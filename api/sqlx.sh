# There's an issue with workspaces, the cli, and compile-time checks. cli commands need the database location to be relative
# while compile time checks use the .env which is relative to the workspace
DATABASE_URL=sqlite://./sqlite.db sqlx ${@}

