# API 仕様

| URI                      | HTTP メソッド | リクエストパラメータ              | レスポンスパラメータ                               |
| ------------------------ | ------------- | --------------------------------- | -------------------------------------------------- |
| localhost:3010/todos     | POST          | {"text": String}                  | {"id": number,"text": String,"completed": false}   |
| localhost:3010/todos     | GET           | None                              | [{"id": number,"text": String,"completed": false}] |
| localhost:3010/todos/:id | GET           | None                              | {"id": number,"text": String,"completed": false}   |
| localhost:3010/todos/:id | PATCH         | {"text":String,"completed": bool} | {"id": number,"text": String,"completed": false}   |
| localhost:3010/todos/:id | DELETE        | None                              | None                                               |

### 後々の話

認証でユーザーごとの Todo リスト
Todo にラベル設定
