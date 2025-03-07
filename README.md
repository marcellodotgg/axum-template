# axum-template
An axum template repository with sqlx, docker, and OAuth set up

## `.env`
```dotenv
DATABASE_URL=sqlite://data.db

# OAuth
GOOGLE_CLIENT_ID=XXXXX-XXXXX.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=XXXXXXX
GOOGLE_CLIENT_REDIRECT=http://localhost:8080/oauth/google/callback

POST_LOGIN_REDIRECT_URL=http://localhost:8080
POST_LOGIN_REDIRECT_ERROR_URL=http://localhost:8080/error

JWT_SECRET=SUPER_DUPER_SECRET
DOMAIN_NAME=""
```

