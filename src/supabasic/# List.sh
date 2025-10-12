# List
curl -v http://localhost:8000/api/address

# Get
curl -v http://localhost:8000/api/address/36b3732c-7a05-471d-a832-906de3fafb9e

# Update (PUT)
curl -X PUT http://localhost:8000/api/address/36b3732c-7a05-471d-a832-906de3fafb9e \
  -H "Content-Type: application/json" \
  -d '{"street_address":"Updated St","city":"Updateville"}'

# Patch (PATCH)
curl -X PATCH http://localhost:8000/api/address/36b3732c-7a05-471d-a832-906de3fafb9e \
  -H "Content-Type: application/json" \
  -d '{"postal_code":"12345"}'

# Delete
curl -X DELETE http://localhost:8000/api/address/36b3732c-7a05-471d-a832-906de3fafb9e

# Resolve (OpenCage)
curl -X POST http://localhost:8000/api/address/1abb3ba3-1a22-4343-ac9e-5056ac72b564/resolve

==========================================

# List
curl -v http://localhost:8000/api/worlds

# Create
curl -X POST http://localhost:8000/api/worlds \
  -H "Content-Type: application/json" \
  -d '{"frame_id":1,"name":"Earth","description":"The canonical base world"}'

# Get
curl -v http://localhost:8000/api/worlds/1

# Update (PUT)
curl -X PUT http://localhost:8000/api/worlds/1 \
  -H "Content-Type: application/json" \
  -d '{"frame_id":1,"name":"Updated Earth","description":"Updated base world"}'

# Patch
curl -X PATCH http://localhost:8000/api/worlds/1 \
  -H "Content-Type: application/json" \
  -d '{"description":"Just testing patch"}'

# Delete
curl -X DELETE http://localhost:8000/api/worlds/1

===================================================================

curl -v http://localhost:8000/api/simulations

curl -v http://localhost:8000/api/simulations/36b5e22e-1284-4924-a2c6-3adc4fd852d5

curl -X POST http://localhost:8000/api/simulations \
  -H "Content-Type: application/json" \
  -d '{
    
    "user_owner_id": null,
    "anon_owner_id": "05594333-1cea-4f54-9b1b-a2360a53692b",
    "tick_rate": 60,
    "last_saved": null,
    "frame_id": 1
  }'


==========================================