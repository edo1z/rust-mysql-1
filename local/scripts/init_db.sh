cd /app
echo "start database setup"
diesel setup
diesel migration run
chown -R 1000:1001 .