use super::{IntoEntity, IntoHandle};
use bevy::prelude::*;
use rapier::data::{ComponentSet, ComponentSetMut, ComponentSetOption, Index};
use rapier::prelude::*;

impl IntoHandle<ColliderHandle> for Entity {
    #[inline]
    fn handle(self) -> ColliderHandle {
        ColliderHandle::from_raw_parts(self.id(), self.generation())
    }
}

impl IntoEntity for ColliderHandle {
    #[inline]
    fn entity(self) -> Entity {
        self.0.entity()
    }
}

pub type QueryPipelineColliderComponentsQuery<'w, 's, 'b> = Query<
    'w, 's,
    (
        Entity,
        &'b ColliderPosition,
        &'b ColliderShape,
        &'b ColliderFlags,
    ),
>;

pub struct QueryPipelineColliderComponentsSet<'w, 's, 'b, 'c>(
    pub &'c QueryPipelineColliderComponentsQuery<'w, 's, 'b>,
);

impl_component_set_wo_query_set!(
    QueryPipelineColliderComponentsSet,
    ColliderPosition,
    |data| data.1
);
impl_component_set_wo_query_set!(QueryPipelineColliderComponentsSet, ColliderShape, |data| {
    data.2
});
impl_component_set_wo_query_set!(QueryPipelineColliderComponentsSet, ColliderFlags, |data| {
    data.3
});

pub struct ColliderComponentsSet<'w, 's, 'b, 'c>(pub ColliderComponentsQuery<'w, 's, 'b, 'c>);

pub type ColliderComponentsQuery<'w, 's, 'b, 'c> = QuerySet<(
    Query<
        'w, 's,
        (
            Entity,
            &'b ColliderChanges,
            &'b ColliderPosition,
            &'b ColliderBroadPhaseData,
            &'b ColliderShape,
            &'b ColliderType,
            &'b ColliderMaterial,
            &'b ColliderFlags,
            Option<&'b ColliderParent>,
        ),
    >,
    Query<
        'w, 's,
        (
            Entity,
            &'c mut ColliderChanges,
            &'c mut ColliderPosition,
            &'c mut ColliderBroadPhaseData,
        ),
    >,
    Query<
        'w, 's,
        (
            Entity,
            &'c mut ColliderChanges,
            Or<(Changed<ColliderPosition>, Added<ColliderPosition>)>,
            Or<(Changed<ColliderFlags>, Added<ColliderFlags>)>,
            Or<(Changed<ColliderShape>, Added<ColliderShape>)>,
            Or<(Changed<ColliderType>, Added<ColliderType>)>,
            Option<Or<(Changed<ColliderParent>, Added<ColliderParent>)>>,
        ),
        Or<(
            Changed<ColliderPosition>,
            Added<ColliderPosition>,
            Changed<ColliderFlags>,
            Added<ColliderFlags>,
            Changed<ColliderShape>,
            Added<ColliderShape>,
            Changed<ColliderType>,
            Added<ColliderType>,
            Changed<ColliderParent>,
            Added<ColliderParent>,
        )>,
    >,
)>;

impl_component_set_mut!(ColliderComponentsSet, ColliderChanges, |data| data.1);
impl_component_set_mut!(ColliderComponentsSet, ColliderPosition, |data| data.2);
impl_component_set_mut!(ColliderComponentsSet, ColliderBroadPhaseData, |d| d.3);

impl_component_set!(ColliderComponentsSet, ColliderShape, |data| data.4);
impl_component_set!(ColliderComponentsSet, ColliderType, |data| data.5);
impl_component_set!(ColliderComponentsSet, ColliderMaterial, |data| data.6);
impl_component_set!(ColliderComponentsSet, ColliderFlags, |data| data.7);

impl_component_set_option!(ColliderComponentsSet, ColliderParent);

#[derive(Bundle)]
pub struct ColliderBundle {
    pub collider_type: ColliderType,
    pub shape: ColliderShape,
    pub position: ColliderPosition,
    pub material: ColliderMaterial,
    pub flags: ColliderFlags,
    pub mass_properties: ColliderMassProps,
    pub changes: ColliderChanges,
    pub bf_data: ColliderBroadPhaseData,
}

impl Default for ColliderBundle {
    fn default() -> Self {
        Self {
            collider_type: ColliderType::Solid,
            shape: ColliderShape::ball(0.5),
            position: ColliderPosition::default(),
            material: ColliderMaterial::default(),
            flags: ColliderFlags::default(),
            mass_properties: ColliderMassProps::default(),
            changes: ColliderChanges::default(),
            bf_data: ColliderBroadPhaseData::default(),
        }
    }
}
