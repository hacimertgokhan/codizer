# https://api.example.com/api/auth/login

**Method**: POST
**Format**: JSON

**Description**: User authentication endpoint

**Tags**: auth

**Consumes**: application/json

**Produces**: application/json

## Parameters

- **username** (body, string)
  - Type: string
  - Description: User login name
  - Required: true


## Responses

### 200
- Description: Login successful
- Schema: {"token": "string"}


## Request Body

```json
{"username": "string"}
```

---

