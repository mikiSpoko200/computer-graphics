use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Display;
use crate::{vertex, program, drawing, index_buffer, uniform};

use crate::gl_assert_no_err;
use uniform::TypedUniform;
use std::ops::Deref;
use program::Program;
use index_buffer::{IndexBuffer, IndexingMode, IndexType};
use crate::index_buffer::IndexBufferObject;
use crate::uniform::{UniformType, UniformTypeProvider};


// fixme: attribute / uniform layout provider - as of now layouts are specified in order.
//      quick solution -> print the manifest of (current layout - glsl lifetime - name)?

pub struct Binder<I = IndexBufferObject>
    where
        I: IndexBuffer
{
    vao: vertex::ArrayObject,
    vbos: Vec<Box<dyn vertex::Buffer>>,
    ebo: IndexingMode<I>,
    program: Program,
    uniforms: HashMap<&'static str, (usize, Box<dyn TypedUniform>)>,
}

impl<I> Binder<I> where I: IndexBuffer,
{
    pub fn new(
        vbos: Vec<Box<dyn vertex::Buffer>>,
        ebo: IndexingMode<I>,
        program: Program,
        uniforms_with_idents: impl Iterator<Item=(&'static str, Box<dyn TypedUniform>)>,
    ) -> Self {
        let vao = vertex::ArrayObject::create();
        let uniforms = HashMap::from_iter(
            uniforms_with_idents.enumerate().map(
                |(index, (ident, uniform))| (ident, (index, uniform))
            )
        );
        Self { vao, vbos, ebo, program, uniforms }
    }

    pub fn uniform_definitions(&self) -> Box<[String]> {
        let mut definitions = self.uniforms.iter()
            .map(| (ident, (index, uniform))|
                format!("layout(location = {}) {} {}", index, uniform.uniform_type(), ident)
            )
            .collect::<Vec<_>>()
            .into_boxed_slice();
        definitions.sort_unstable();
        definitions
    }

    // todo: scoped_binder controls if appropriate object is already bound if so it returns null binder of sort.
    //      uniform indexes or more generally should be provided and managed by and external object.

    fn bind_uniforms(&self) {
        for (index, uniform) in self.uniforms.values() {
            uniform.bind(*index as _);
        }
    }

    pub fn add_uniform(&mut self, ident: &'static str, uniform: Box<dyn TypedUniform>) {
        // todo: index is never stored and depends on the order in uniforms - this is terrible fix it please xoxo
        let new_index = self.uniforms.len();
        let entry = self.uniforms.entry(ident);
        match entry {
            Entry::Occupied(_) => panic!("uniform with ident {} already exists", ident),
            Entry::Vacant(location) => location.insert((new_index, uniform)),
        };
    }

    pub fn update_uniform(&mut self, ident: &'static str, uniform: Box<dyn TypedUniform>) {
        let _program_binder = self.program_binder();
        let mut entry = self.uniforms.entry(ident);
        match entry {
            Entry::Occupied(ref mut location) => {
                let (index, _) = location.get();
                uniform.bind(*index as _);
                location.insert((*index, uniform));
            }
            Entry::Vacant(_) => panic!("Uniform {} was not registered.", ident),
        }

    }

    pub fn upload(&mut self) {
        let _program_scoped_binder = self.program.scoped_binder();

        self.bind_uniforms();
        println!("Uniform definitions for program: {}", self.program.id());
        for uni_def in self.uniform_definitions().iter() {
            println!("\t{}", uni_def);
        }

        let _vao_binder = self.vao.scoped_binder();
        for (index, vbo) in self.vbos.iter().enumerate() {
            let _scoped_binder = vbo.as_ref().scoped_binder();
            gl_assert_no_err!();
            vbo.as_ref().upload();
            gl_assert_no_err!();
            self.vao.set_vertex_attrib_pointer(index as _, &vbo.as_ref().attribute_type());
            gl_assert_no_err!();
        }

        if let Some(ref index_buffer) = self.ebo {
            let _ebo_binder = index_buffer.scoped_binder();
            index_buffer.upload();
        }
    }

    pub fn vertex_count(&self) -> usize {
        // vertex count provider trait
        if let Some(ref index_buffer) = self.ebo {
            index_buffer.vertex_count()
        } else if let Some(vbo) = self.vbos.first() {
            vbo.as_ref().vertex_count()
        } else {
            2
        }
    }

    pub fn vao(&self) -> &vertex::array_object::ArrayObject {
        &self.vao
    }

    pub fn vao_binder(&self) -> vertex::array_object::ScopedBinder {
        self.vao.scoped_binder()
    }

    pub fn index_type(&self) -> Option<IndexType> {
        // index_type provider
        self.ebo.as_ref().map(|index_buffer| index_buffer.index_type())
    }

    pub fn program_binder(&self) -> program::ScopedBinder { self.program.scoped_binder() }

    pub fn draw_binder(&self) -> DrawScopedBinder {
        DrawScopedBinder::new(self.program_binder(), self.vao_binder())
    }
}

pub struct DrawScopedBinder(program::ScopedBinder, vertex::array_object::ScopedBinder);

impl DrawScopedBinder {
    pub fn new(program: program::ScopedBinder, vao: vertex::array_object::ScopedBinder) -> Self {
        Self(program, vao)
    }
}
