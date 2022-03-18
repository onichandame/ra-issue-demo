use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct TestDTO {
    pub id: i32,
}

#[derive(SimpleObject)]
pub struct TestDTO2 {
    pub dummy: Dummy,
}

pub struct Dummy {}
