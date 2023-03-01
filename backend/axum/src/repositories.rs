use std::{
    collections::HashMap,
    sync::{Arc,RwLock,RwLockReadGuard,RwLockWriteGuard},
    };
use serde::{Deserialize,Serialize};
use thiserror::Error;
use anyhow::{Context, Ok};
use tokio::time::error;
use validator::Validate;
use axum::async_trait;
use sqlx::{PgPool,FromRow};

#[derive(Debug,Error)]
enum RepositoryError {
    #[error("Not found, id is {0}")]
    NotFound(i32),
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String)
}

#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self,payload:CreateTodo) -> anyhow::Result<Todo>;
    async fn find(&self,id:i32) -> anyhow::Result<Todo>;
    async fn all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn update(&self,id:i32,payload:UpdateTodo) -> anyhow::Result<Todo>;
    async fn delete(&self,id:i32) -> anyhow::Result<()>;
}

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Eq,FromRow)]
pub struct Todo {
    pub id:i32,
    pub text:String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id:i32,text:String) -> Self {
        Self { 
            id, 
            text, 
            completed: false }
    }
}

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Eq,Validate)]

pub struct CreateTodo {
#[validate(length(min = 1, message = "Can not be empty"))]
#[validate(length(max = 100, message = "Over text length"))]
    text:String,
}

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Eq,Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    text:Option<String>,
    completed:Option<bool>,
}

type TodoDatas = HashMap<i32,Todo>;

#[derive(Debug,Clone)]
pub struct TodoRepositoryForMemory {
    store:Arc<RwLock<TodoDatas>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory { store: Arc::default() }
    }
    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDatas> {
        self.store.write().unwrap()
    }
    fn read_store_ref(&self) -> RwLockReadGuard<TodoDatas> {
        self.store.read().unwrap()
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForMemory {
    async fn create(&self,payload:CreateTodo) -> anyhow::Result<Todo>{
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let todo = Todo::new(id, payload.text.clone());
        store.insert(id,todo.clone());
        Ok(todo)
    }
    async fn find(&self,id:i32) -> anyhow::Result<Todo> {
        let store = self.read_store_ref();
        let todo = store.get(&id).map(|todo| todo.clone()).ok_or(RepositoryError::NotFound(id))?;
        Ok(todo)
    }
    async fn all(&self) -> anyhow::Result<Vec<Todo>> {
        let store = self.read_store_ref();
        Ok(Vec::from_iter(store.values().cloned()))
    }
    async fn update(&self,id:i32,payload:UpdateTodo) -> anyhow::Result<Todo> {
        let mut store = self.write_store_ref();
        let todo = store.get(&id).context(RepositoryError::NotFound(id))?;
        let text = payload.text.unwrap_or(todo.text.clone());
        let completed = payload.completed.unwrap_or(todo.completed);
        let todo = Todo {id,text,completed};
        store.insert(id, todo.clone());
        Ok(todo)
    }
    async fn delete(&self,id:i32) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}

#[derive(Debug,Clone)]
pub struct TodoRepositoryForDb {
    pool: PgPool,
}

impl  TodoRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        TodoRepositoryForDb { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForDb {
    async fn create(&self,_payload: CreateTodo) -> anyhow::Result<Todo> {
        let todo = sqlx::query_as::<_,Todo> (
            r#"
                insert into todos (text , completed) values ($1,false)
                returning *
            "#,)
            .bind(_payload.text.clone())
            .fetch_one(&self.pool)
            .await?;

        Ok(todo)
        }
    async fn find(&self,id:i32) -> anyhow::Result<Todo> {
        let todo = sqlx::query_as::<_,Todo> (
            r#"
                select * from todos where id = $1
            "#,)
            .bind(&id)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(todo)
        }
    async fn all(&self) -> anyhow::Result<Vec<Todo>> {
        let todo = sqlx::query_as::<_,Todo>(
            r#"
                select * from todos order by id desc;
            "#)
            .fetch_all(&self.pool)
            .await?;

        Ok(todo)
        }
        async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
            let old_todo = self.find(id).await?;
            let todo = sqlx::query_as::<_, Todo>(
                r#"
    update todos set text=$1, completed=$2
    where id=$3
    returning *
            "#,
            )
            .bind(payload.text.unwrap_or(old_todo.text))
            .bind(payload.completed.unwrap_or(old_todo.completed))
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
    
            Ok(todo)
        }
    async fn delete(&self,id:i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                delete from todos where id=$1
            "#,)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string())
        })?;

        Ok(())
    }
}