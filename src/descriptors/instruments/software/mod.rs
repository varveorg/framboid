pub mod abstraction;
pub mod api;
pub mod dependency;
pub mod modularization;
pub mod naming;
pub mod optimization;
pub mod resilience;
pub mod testing;

use crate::descriptors::instruments::software::{abstraction::AbstractionInstrument, api::ApiInstrument, dependency::DependencyInstrument, modularization::ModularizationInstrument, naming::NamingInstrument, optimization::OptimizationInstrument, resilience::ResilienceInstrument, testing::TestingInstrument};

pub enum SoftwareInstrument {
    Abstraction(AbstractionInstrument),
    Api(ApiInstrument),
    Dependency(DependencyInstrument),
    Modularization(ModularizationInstrument),
    Naming(NamingInstrument),
    Optimization(OptimizationInstrument),
    Resilience(ResilienceInstrument),
    Testing(TestingInstrument)
}