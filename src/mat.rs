
use vec::{
    bvec2, bvec3, bvec4,
    ivec2, ivec3, ivec4,
    uvec2, uvec3, uvec4,
     vec2,  vec3,  vec4,
    dvec2, dvec3, dvec4,
};

pub type bmat2x2 = array![bvec2; 2];
pub type bmat2x3 = array![bvec3; 2];
pub type bmat2x4 = array![bvec4; 2];
pub type bmat3x2 = array![bvec2; 3];
pub type bmat3x3 = array![bvec3; 3];
pub type bmat3x4 = array![bvec4; 3];
pub type bmat4x2 = array![bvec2; 4];
pub type bmat4x3 = array![bvec3; 4];
pub type bmat4x4 = array![bvec4; 4];
pub type bmat2 = bmat2x2;
pub type bmat3 = bmat3x3;
pub type bmat4 = bmat4x4;

pub type imat2x2 = array![ivec2; 2];
pub type imat2x3 = array![ivec3; 2];
pub type imat2x4 = array![ivec4; 2];
pub type imat3x2 = array![ivec2; 3];
pub type imat3x3 = array![ivec3; 3];
pub type imat3x4 = array![ivec4; 3];
pub type imat4x2 = array![ivec2; 4];
pub type imat4x3 = array![ivec3; 4];
pub type imat4x4 = array![ivec4; 4];
pub type imat2 = imat2x2;
pub type imat3 = imat3x3;
pub type imat4 = imat4x4;

pub type umat2x2 = array![uvec2; 2];
pub type umat2x3 = array![uvec3; 2];
pub type umat2x4 = array![uvec4; 2];
pub type umat3x2 = array![uvec2; 3];
pub type umat3x3 = array![uvec3; 3];
pub type umat3x4 = array![uvec4; 3];
pub type umat4x2 = array![uvec2; 4];
pub type umat4x3 = array![uvec3; 4];
pub type umat4x4 = array![uvec4; 4];
pub type umat2 = umat2x2;
pub type umat3 = umat3x3;
pub type umat4 = umat4x4;

pub type mat2x2 = array![vec2; 2];
pub type mat2x3 = array![vec3; 2];
pub type mat2x4 = array![vec4; 2];
pub type mat3x2 = array![vec2; 3];
pub type mat3x3 = array![vec3; 3];
pub type mat3x4 = array![vec4; 3];
pub type mat4x2 = array![vec2; 4];
pub type mat4x3 = array![vec3; 4];
pub type mat4x4 = array![vec4; 4];
pub type mat2 = mat2x2;
pub type mat3 = mat3x3;
pub type mat4 = mat4x4;

pub type dmat2x2 = array![dvec2; 2];
pub type dmat2x3 = array![dvec3; 2];
pub type dmat2x4 = array![dvec4; 2];
pub type dmat3x2 = array![dvec2; 3];
pub type dmat3x3 = array![dvec3; 3];
pub type dmat3x4 = array![dvec4; 3];
pub type dmat4x2 = array![dvec2; 4];
pub type dmat4x3 = array![dvec3; 4];
pub type dmat4x4 = array![dvec4; 4];
pub type dmat2 = dmat2x2;
pub type dmat3 = dmat3x3;
pub type dmat4 = dmat4x4;


