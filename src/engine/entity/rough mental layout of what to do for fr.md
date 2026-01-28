rough mental layout of what to do for frontned.

SimEntity: 
 <--id: EntityId, 
|  world_id: WorldId ------> World Selection/ World Context--->|(world)
|   position: UvoxId ------------------------------------------>|(position)
|   orientation: Quat ----------------------------------------->|(orientation)
|   spawned_at: SimTime ----------------------->TimeContext->|  |
|   template: Objex ---->Material + Geometry                 |  |
|                          |              |                  | PropertyContext
|                   Material Selection    GeometryEditor     |  |
|                           |                    |           |  PropertyEditorCanvas
|                           |                    |           |  |
|                           --------------->ObjexCanvas      -->|(spawned_at)
|                                               |               |
|                                             Objex------------>|(template)
|                                                               |
-----------------------------------------Generate EntityId----->|
                                                                |->SimEntity Save/Push/Load to Simulated world. 