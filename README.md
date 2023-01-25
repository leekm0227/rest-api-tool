# rest-api-tool

## example

model.yaml
```
memo:
  fields:
    content: str

todo:
  fields:
    is_complete: bool
    content: str
    memos: memo[]
  methods: c,r,u,d
  ```
  
  
  endpoints
  ```
  [GET]
  /todo
  /todo/{todo_id}
  /todo/{todo_id}/memo
  
  [POST]
  /todo
  /todo/{todo_id}/memo
  
  [PATCH, DELETE]
  /todo/{todo_id}
  /todo/{todo_id}/memo/{memo_id}
  ```
