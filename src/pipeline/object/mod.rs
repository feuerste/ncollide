//! Definition of collision objects and some of their properties.

pub use self::collision_groups::{CollisionGroups, CollisionGroupsPairFilter};
pub use self::collision_object::{
    CollisionObjectRef, CollisionObject, CollisionObjectSlabHandle, CollisionObjectUpdateFlags,
};
pub use self::collision_object_set::{CollisionObjectSet, CollisionObjectSlab, CollisionObjects, CollisionObjectHandle};
pub use self::query_type::GeometricQueryType;


mod collision_object;
mod collision_object_set;
mod collision_groups;
mod query_type;