-react$ curl -i http://loccurl -i http://localhost:8000/api/session/init
HTTP/1.1 200 OK
content-type: application/json
vary: origin, access-control-request-method, access-control-request-headers
vary: origin, access-control-request-method, access-control-request-headers
access-control-allow-origin: *
access-control-expose-headers: *
content-length: 193
date: Mon, 26 Jan 2026 08:30:55 GMT

{"reused":false,
    "session":
    {"is_anon":true,
        "property_id":null,
        "user_id":"7948a566-b82c-4592-a6fc-971d700a785d",
        "world_id":null},
        "session_id":"6f44c1d9-3535-4c20-9953-6ab673e2892b",
        "status":"ok"}
        
        curl -i \
  -H "x-session-id: 6f44c1d9-3535-4c20-9953-6ab673e2892b" \
  -H "x-user-id: 7948a566-b82c-4592-a6fc-971d700a785d" \
  -X POST \
  http://localhost:8000/api/session/world/1

  curl -i \
  -H "x-session-id: 6f44c1d9-3535-4c20-9953-6ab673e2892b" \
  http://localhost:8000/api/session/status


  curl -i \
  -H "x-session-id: b787cdc2-82a5-4228-bc34-792d99f797ed" \
  -H "x-user-id: da9696aa-b601-47fa-93e7-3723be4954a2" \
  -X POST \
  http://localhost:8000/api/session/world/1

curl \
-X POST \
http://localhost:8000/api/auth/signup \
{
  "email": "test@example.com",
  "password": "password",
  "display_name": "Test User"
}

curl -i \
  -X POST \
  -H "Content-Type: application/json" \
  -H "x-session-id: 689c78b5-abab-47d6-be1c-6f1beed8dc11" \
  http://localhost:8000/api/auth/signup \
  -d '{
    "email": "test@example.com",
    "password": "password",
    "display_name": "Test User"
  }'

{"reused":false,"session":{"is_anon":true,"property_id":null,"user_id":"ab2bdba7-8941-4c25-8e05-bdaede53afff","world_id":null},"session_id":"689c78b5-abab-47d6-be1c-6f1beed8dc11","status":"ok"}drippy@JDPlumbing:~/jdplumbing-app/rust/omnivox-react$ 




{"reused":false,"session":{"is_anon":true,"property_id":null,"user_id":"c267bb4e-ae56-4ed9-acba-cfb5ef3506ec","world_id":null},"session_id":"10cbfe48-61e1-491f-ae14-981b650622de","status":"ok"}drippy@JDPlumbing:~/jdplumbing-app/rust/omnivox-re


curl -X POST \
-H "Content-Type: application/json" \
-H "x-session-id: 04a93711-4d73-4c59-952b-67847471e134" \
http://localhost:8000/api/auth/signup \
-d '{
  "email": "test@example.com",
  "password": "password",
  "display_name": "Test User"
}'

curl -X POST \
  -H "Content-Type: application/json" \
  -H "x-session-id: 1ac61a6c-ee15-4989-b52b-1312dac02051" \
  http://localhost:8000/api/auth/signup \
  -d '{
    "email": "finaltest@example.com",
    "password": "password",
    "display_name": "final test User"
  }'
curl \
  -H "x-session-id: 52861b28-347a-49da-b06a-5b1583fdf88b" \
  http://localhost:8000/api/session/status





{"reused":false,"session":{"anon_owner_id":"8aa215bf-9218-4f1b-af63-78f25fc71333","is_anon":true,"property_id":null,"user_id":"7dcca10d-0090-4de5-97d8-691b8ed97427","world_id":null},"session_id":"1ac61a6c-ee15-4989-b52b-1312dac02051","status":"ok"}drippy@JDPlumbing:~/jdplumbing-app/rust/omnivox-react$ 