# My basic tools for Bevy projects

## Features

* autodespawn
  * Components
    * AutoDespawn
      * frames_left: u32
      * seconds_left: u32
  * Systems
    * auto_despawn_system
      * Counts frames_left and seconds_left of AutoDespawn down.  When both are zero, the entity will be despawned.

* health
  * Components
    * Health
      * current: f32
      * max: f32
    * Dead
  * Systems
    * death_check_system
      * if Health.current <= 0, remove Health and apply Dead.
    * auto_despawn_death
      * Requires autodespawn feature
      * Will assign AutoDespawn with frames_left = 2 to a Dead entity
  
* mana
  * Components
    * Mana
      * current: f32
      * max: f32

* stamina
  * Components
    * Stamina
      * current: f32
      * max: f32
      * refill: f32
      * cooldown: boolean
  * Systems
    * stamina_refill_system
      * Refills Stamina.refill per second of Stamina.current 
    * stamina_cooldown_system
      * If stamina is <= 0, the cool down flag will be set.  If >= max, the cooldown flag will be unset.  During cooldown, stamina should not be allowed be used.

* damage
  * Enables features
    * health
  * Crates
    * heron
  * Components
    * Damager 
      * strength: f32
    * Damagable
      * resistence: f32
  * Systems
    * damage_system
      * If a Damager collides with a Damagable, it will reduce the Damagable's health by strength - resist per second.

* direction
  * Components
    * Direction (enum)
      * North
      * South
      * East
      * West

* walking
  * Enables features
    * direction
  * Crates
    * heron
  * Components 
    * Walk
      * speed: f32
      * walking: boolean
  * System
    * walking_system
      * Sets the velocity depending of the Direction, walk speed and if walking is on

* running
  * Enables features
    * walking
    * direction
  * Creates
    * heron
  * Components
    * Run
      * speed: f32
      * running: boolean
  * System
    * running_system
      * Sets the velocity depending of the Direction, run speed and if running is on.  If stamina feature is on and Stamina component is assigned, it will reduce the stamina 1 per second and stop if stamina cooldown flag is set.
  
* player
  * Components
    * Player


* spritessheet
  * Trait
    * Filenames
      * get_filenames() -> Vec<String>
  * Systems
    * initialize_loading

