## Install 

```shell
git clone git@github.com:web3ten0/rust-mysql-1.git
cd rust-mysql-1/local
docker-compose up -d
sh scripts/exec_init_db
```

## Migration

```shell
cd rust-mysql-1/local
sh scripts/migrate run
```

```shell
cd rust-mysql-1/local
sh scripts/migrate generate ${table name}
```