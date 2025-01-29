# POST
curl -X POST -H "Content-Type: application/json" \
-d '{"name": "Teclado", "quantity": 5}' \
http://127.0.0.1:8080/items

# GET
curl http://127.0.0.1:8080/items