KEY=$(cat key)
curl    -X POST \
        -H "Content-Type: application/json" \
        -H "x-api-key: $KEY" \
        --data "{\"phone\":\"$1\",\"message\":\"$2\", \"device_id\": \"curl-test-agent\"}" \
        http://localhost:8080/send