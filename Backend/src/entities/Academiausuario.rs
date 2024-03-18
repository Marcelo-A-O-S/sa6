#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(Id))]
pub struct Academiausuario {
    pub Id: i32,
    pub AcademiaId: Option<i32>,
    pub UsuarioId: Option<i32>,
}