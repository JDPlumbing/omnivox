MATH
  └─ PHYSICS (pure laws)
       └─ COSMIC (state: radiation, gravity, pose)

            └─ WORLD (interpretation: insolation, tides, seasons)
                 └─ ENVIRONMENT (effects: temp, pressure, density)
                      └─ ECS / ENTITIES


COSMICBODYID: has: components related to PHYSICS
               systems output values based on components
                    

WORLDID: has: components related to cosmic body
                    systems output environmental values based on location within that world. 
          
LOCATION: has: components dependent on world location based on time (day vs night, summber vs winter, etc)

