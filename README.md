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
  /todo
  /todo/{todo_id}
  /todo/{todo_id}/memo
  /todo/{todo_id}/memo/{memo_id}
  
  methods: GET, POST, PATCH, DELETE
  ```
