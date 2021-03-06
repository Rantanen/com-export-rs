use intercom::*;

com_module!(
    class RefCountOperations,
    class ClassCreator,
    class CreatedClass,
);

#[com_interface]
trait IRefCount
{
    fn get_ref_count(&self) -> u32;
}

#[com_class(ClassCreator)]
#[derive(Default)]
pub struct ClassCreator {}

#[com_interface]
impl ClassCreator
{
    pub fn create_root(&self, id: i32) -> ComResult<ComRc<CreatedClass>>
    {
        Ok(ComRc::from(&ComBox::new(CreatedClass::new_with_id(id))))
    }

    pub fn create_child(
        &self,
        id: i32,
        parent: &ComItf<dyn IParent>,
    ) -> ComResult<ComRc<CreatedClass>>
    {
        Ok(ComRc::from(&ComBox::new(CreatedClass::new_child(
            id,
            parent.get_id(),
        ))))
    }
}

#[com_class(CreatedClass, IParent, IRefCount)]
#[derive(Default)]
pub struct CreatedClass
{
    id: i32,
    parent: i32,
}

#[com_interface]
impl CreatedClass
{
    pub fn new_with_id(id: i32) -> CreatedClass
    {
        CreatedClass { id, parent: 0 }
    }
    pub fn new_child(id: i32, parent: i32) -> CreatedClass
    {
        CreatedClass { id, parent }
    }

    pub fn get_id(&self) -> ComResult<i32>
    {
        Ok(self.id)
    }
    pub fn get_parent_id(&self) -> ComResult<i32>
    {
        Ok(self.parent)
    }
}

impl IRefCount for CreatedClass
{
    fn get_ref_count(&self) -> u32
    {
        let combox = unsafe { ComBoxData::of(self) };
        combox.get_ref_count()
    }
}

#[com_interface]
pub trait IParent
{
    fn get_id(&self) -> i32;
}

impl IParent for CreatedClass
{
    fn get_id(&self) -> i32
    {
        self.id
    }
}

#[com_class(RefCountOperations)]
#[derive(Default)]
pub struct RefCountOperations {}

#[com_interface]
impl RefCountOperations
{
    pub fn get_new(&self) -> ComResult<ComRc<RefCountOperations>>
    {
        Ok(ComRc::from(&ComBox::<RefCountOperations>::default()))
    }

    pub fn get_ref_count(&self) -> u32
    {
        let combox = unsafe { ComBoxData::of(self) };
        combox.get_ref_count()
    }
}
