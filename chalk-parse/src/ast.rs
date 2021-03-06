use lalrpop_intern::InternedString;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub fn new(lo: usize, hi: usize) -> Self {
        Span { lo: lo, hi: hi }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Program {
    pub items: Vec<Item>
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Item {
    StructDefn(StructDefn),
    TraitDefn(TraitDefn),
    Impl(Impl),
    Clause(Clause),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StructDefn {
    pub name: Identifier,
    pub parameter_kinds: Vec<ParameterKind>,
    pub where_clauses: Vec<QuantifiedWhereClause>,
    pub fields: Vec<Field>,
    pub flags: StructFlags,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StructFlags {
    pub external: bool,
    pub fundamental: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TraitDefn {
    pub name: Identifier,
    pub parameter_kinds: Vec<ParameterKind>,
    pub where_clauses: Vec<QuantifiedWhereClause>,
    pub assoc_ty_defns: Vec<AssocTyDefn>,
    pub flags: TraitFlags,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TraitFlags {
    pub auto: bool,
    pub marker: bool,
    pub external: bool,
    pub deref: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AssocTyDefn {
    pub name: Identifier,
    pub parameter_kinds: Vec<ParameterKind>,
    pub bounds: Vec<InlineBound>,
    pub where_clauses: Vec<QuantifiedWhereClause>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ParameterKind {
    Ty(Identifier),
    Lifetime(Identifier),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Parameter {
    Ty(Ty),
    Lifetime(Lifetime),
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// An inline bound, e.g. `: Foo<K>` in `impl<K, T: Foo<K>> SomeType<T>`.
pub enum InlineBound {
    TraitBound(TraitBound),
    ProjectionEqBound(ProjectionEqBound),
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// Represents a trait bound on e.g. a type or type parameter.
/// Does not know anything about what it's binding.
pub struct TraitBound {
    pub trait_name: Identifier,
    pub args_no_self: Vec<Parameter>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// Represents a projection equality bound on e.g. a type or type parameter.
/// Does not know anything about what it's binding.
pub struct ProjectionEqBound {
    pub trait_bound: TraitBound,
    pub name: Identifier,
    pub args: Vec<Parameter>,
    pub value: Ty,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Kind {
    Ty,
    Lifetime,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            match *self {
                Kind::Ty => "type",
                Kind::Lifetime => "lifetime",
            }
        )
    }
}

pub trait Kinded {
    fn kind(&self) -> Kind;
}

impl Kinded for ParameterKind {
    fn kind(&self) -> Kind {
        match *self {
            ParameterKind::Ty(_) => Kind::Ty,
            ParameterKind::Lifetime(_) => Kind::Lifetime,
        }
    }
}

impl Kinded for Parameter {
    fn kind(&self) -> Kind {
        match *self {
            Parameter::Ty(_) => Kind::Ty,
            Parameter::Lifetime(_) => Kind::Lifetime,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Impl {
    pub parameter_kinds: Vec<ParameterKind>,
    pub trait_ref: PolarizedTraitRef,
    pub where_clauses: Vec<QuantifiedWhereClause>,
    pub assoc_ty_values: Vec<AssocTyValue>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AssocTyValue {
    pub name: Identifier,
    pub parameter_kinds: Vec<ParameterKind>,
    pub value: Ty,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Ty {
    Id {
        name: Identifier,
    },
    Apply {
        name: Identifier,
        args: Vec<Parameter>
    },
    Projection {
        proj: ProjectionTy,
    },
    UnselectedProjection {
        proj: UnselectedProjectionTy,
    },
    ForAll {
        lifetime_names: Vec<Identifier>,
        ty: Box<Ty>
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Lifetime {
    Id {
        name: Identifier,
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProjectionTy {
    pub trait_ref: TraitRef,
    pub name: Identifier,
    pub args: Vec<Parameter>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UnselectedProjectionTy {
    pub name: Identifier,
    pub args: Vec<Parameter>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TraitRef {
    pub trait_name: Identifier,
    pub args: Vec<Parameter>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PolarizedTraitRef {
    Positive(TraitRef),
    Negative(TraitRef),
}

impl PolarizedTraitRef {
    pub fn from_bool(polarity: bool, trait_ref: TraitRef) -> PolarizedTraitRef {
        if polarity {
            PolarizedTraitRef::Positive(trait_ref)
        } else {
            PolarizedTraitRef::Negative(trait_ref)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Identifier {
    pub str: InternedString,
    pub span: Span,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WhereClause {
    Implemented { trait_ref: TraitRef },
    ProjectionEq { projection: ProjectionTy, ty: Ty },
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DomainGoal {
    Holds { where_clause: WhereClause },
    Normalize { projection: ProjectionTy, ty: Ty },
    TraitRefWellFormed { trait_ref: TraitRef },
    TyWellFormed { ty: Ty },
    TyFromEnv { ty: Ty },
    TraitRefFromEnv { trait_ref: TraitRef },
    TraitInScope { trait_name: Identifier },
    Derefs { source: Ty, target: Ty },
    IsLocal { ty: Ty },
    IsExternal { ty: Ty },
    IsDeeplyExternal { ty: Ty },
    LocalImplAllowed { trait_ref: TraitRef },
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LeafGoal {
    DomainGoal { goal: DomainGoal },
    UnifyTys { a: Ty, b: Ty },
    UnifyLifetimes { a: Lifetime, b: Lifetime },
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuantifiedWhereClause {
    pub parameter_kinds: Vec<ParameterKind>,
    pub where_clause: WhereClause,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Field {
    pub name: Identifier,
    pub ty: Ty,
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// This allows users to add arbitrary `A :- B` clauses into the
/// logic; it has no equivalent in Rust, but it's useful for testing.
pub struct Clause {
    pub parameter_kinds: Vec<ParameterKind>,
    pub consequence: DomainGoal,
    pub conditions: Vec<Box<Goal>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Goal {
    ForAll(Vec<ParameterKind>, Box<Goal>),
    Exists(Vec<ParameterKind>, Box<Goal>),
    Implies(Vec<Clause>, Box<Goal>),
    And(Box<Goal>, Box<Goal>),
    Not(Box<Goal>),

    // Additional kinds of goals:
    Leaf(LeafGoal),
}
