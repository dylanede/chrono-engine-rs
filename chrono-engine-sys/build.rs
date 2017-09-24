extern crate cmake;
extern crate bindgen;

fn main() {

    let mut cfg = cmake::Config::new("chrono");
    cfg.define("BUILD_DEMOS", "OFF")
        .define("BUILD_TESTING", "OFF")
        .profile("Release");
    if cfg!(feature = "cascade") {
        cfg.define("ENABLE_MODULE_CASCADE", "ON");
    }
    if cfg!(feature = "cosimulation") {
        cfg.define("ENABLE_MODULE_COSIMULATION", "ON");
    }
    if cfg!(feature = "fea") {
        cfg.define("ENABLE_MODULE_FEA", "ON");
    }
    if cfg!(feature = "fsi") {
        cfg.define("ENABLE_MODULE_FSI", "ON");
    }
    if cfg!(feature = "irrlicht") {
        cfg.define("ENABLE_MODULE_IRRLICHT", "ON");
    }
    if cfg!(feature = "matlab") {
        cfg.define("ENABLE_MODULE_MATLAB", "ON");
    }
    if cfg!(feature = "mkl") {
        cfg.define("ENABLE_MODULE_MKL", "ON");
    }
    if cfg!(feature = "opengl") {
        cfg.define("ENABLE_MODULE_OPENGL", "ON");
    }
    if cfg!(feature = "parallel") {
        cfg.define("ENABLE_MODULE_PARALLEL", "ON");
    }
    if cfg!(feature = "postprocess") {
        cfg.define("ENABLE_MODULE_POSTPROCESS", "ON");
    }
    if cfg!(feature = "python") {
        cfg.define("ENABLE_MODULE_PYTHON", "ON");
    }
    if cfg!(feature = "vehicle") {
        cfg.define("ENABLE_MODULE_VEHICLE", "ON");
    }
    let chrono_dst = cfg.build();
    let lib_dir = std::fs::read_dir(&chrono_dst).unwrap()
            .map(|item| item.unwrap().path())
            .filter(|path| path.file_name().and_then(|oss| oss.to_str()).map(|s| s.starts_with("lib")).unwrap_or(false))
            .next().unwrap();
    let include_dir = chrono_dst.join("include");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=ChronoEngine");
    println!("cargo:include-dir={}", include_dir.display());
    println!("cargo:lib-dir={}", lib_dir.display());
    println!("cargo:bin-dir={}", chrono_dst.join("bin").display());

    static WRAPPER_H: &'static str = r#"
#include "chrono/core/ChMatrixNM.h"
#include "chrono/core/ChMatrix33.h"
#include "chrono/physics/ChSystem.h"
#include "chrono/physics/ChSystemNSC.h"
#include "chrono/physics/ChBody.h"
#include "chrono/collision/ChCCollisionModel.h"

/**
 * <div rustbindgen replaces="chrono::ChMatrix33"></div>
 */
template<typename Real>
class MyMatrix33 : public chrono::ChMatrix<Real>  {
#ifdef CHRONO_HAS_AVX
    Real buffer[3 * 3 + 3];
#else
    Real buffer[3 * 3];
#endif
};

/**
 * <div rustbindgen replaces="std::unique_ptr"></div>
 */
template<typename T>
class MyUniquePtr {
    T* ptr;
};
"#;

    use std::env;
    use std::path::PathBuf;

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let msvc = target.contains("msvc");

    let mut builder = bindgen::Builder::default()
        .clang_args(vec![format!("-I{}", include_dir.display()), "-std=c++14".to_string()]);

    if msvc {
        builder = builder.clang_arg("-fms-compatibility-version=19");
    }

    builder = builder
        .opaque_type("std::list.*")
        .opaque_type("std::unordered.*")
        .opaque_type("std::.*string.*")
        .opaque_type("std::_String.*")
        .opaque_type("std::.*stream.*")
        .hide_type("std::vector.*");

    for &class_name in CHRONO_CLASS_NAMES {
        let s = format!("chrono::{}", class_name);
        builder = builder
            .whitelisted_type(&s);
        if TRANSPARENT_CHRONO_CLASS_NAMES.binary_search(&class_name).is_err() {
            builder = builder.opaque_type(&s);
        }
    }

    builder = builder
        .ignore_functions()
        .ignore_methods()
        .with_codegen_config(bindgen::CodegenConfig {
            functions: false,
            types: true,
            vars: true,
            methods: false,
            constructors: false,
            destructors: false
        })
        .derive_debug(false)
        .enable_cxx_namespaces()
        .header_contents("wrapper.hpp", WRAPPER_H);

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

static TRANSPARENT_CHRONO_CLASS_NAMES: &[&'static str] = &[
    "ChCoordsys",
    "ChMatrix",
    "ChMatrix33",
    "ChMatrixDynamic",
    "ChMatrixNM",
    "ChQuaternion",
    "ChVector",
    "ChVector2",
    "ChVectorDynamic",
];

static CHRONO_CLASS_NAMES: &[&'static str] = &[
    "collision::ChCollisionModel",
    "Ch3DOFContainer",
    "ChAparticle",
    "ChArchive",
    "ChArchiveAsciiDump",
    "ChArchiveIn",
    "ChArchiveInBinary",
    "ChArchiveInJSON",
    "ChArchiveOut",
    "ChArchiveOutBinary",
    "ChArchiveOutJSON",
    "ChAssembly",
    "ChAssemblyAnalysis",
    "ChAsset",
    "ChAssetLevel",
    "ChBezierCurve",
    "ChBezierCurveTracker",
    "ChBinaryArchive",
    "ChBody",
    "ChBodyAuxRef",
    "ChBodyEasyBox",
    "ChBodyEasyClusterOfSpheres",
    "ChBodyEasyConvexHull",
    "ChBodyEasyConvexHullAuxRef",
    "ChBodyEasyCylinder",
    "ChBodyEasyEllipsoid",
    "ChBodyEasyMesh",
    "ChBodyEasySphere",
    "ChBodyFrame",
    "ChBoxShape",
    "ChCamera",
    "ChCapsuleShape",
    "ChClassFactory",
    "ChClassRegistration",
    "ChClassRegistrationBase",
    "ChColor",
    "ChColorAsset",
    "ChConeShape",
    "ChConstantDistribution",
    "ChConstraint",
    "ChConstraintBilateral",
    "ChConstraintRigidRigid",
    "ChConstraintThree",
    "ChConstraintThreeBBShaft",
    "ChConstraintThreeGeneric",
    "ChConstraintTuple_1vars",
    "ChConstraintTuple_2vars",
    "ChConstraintTuple_3vars",
    "ChConstraintTuple_4vars",
    "ChConstraintTwo",
    "ChConstraintTwoBodies",
    "ChConstraintTwoGeneric",
    "ChConstraintTwoGenericBoxed",
    "ChConstraintTwoTuples",
    "ChConstraintTwoTuplesContactN",
    "ChConstraintTwoTuplesContactNall",
    "ChConstraintTwoTuplesFrictionT",
    "ChConstraintTwoTuplesFrictionTall",
    "ChConstraintTwoTuplesRollingN",
    "ChConstraintTwoTuplesRollingNall",
    "ChConstraintTwoTuplesRollingT",
    "ChConstraintTwoTuplesRollingTall",
    "ChContactable",
    "ChContactContainer",
    "ChContactContainerNSC",
    "ChContactContainerParallel",
    "ChContactContainerSMC",
    "ChContactNSC",
    "ChContactNSCrolling",
    "ChContactSMC",
    "ChContactTuple",
    "ChContinuumDistribution",
    "ChContinuumSPH",
    "ChControllerPID",
    "ChControls",
    "ChConveyor",
    "ChCoordsys",
    "ChCSMatrix",
    "ChCubicSpline",
    "ChCylinderShape",
    "ChDiscreteDistribution",
    "ChDistribution",
    "ChEllipsoidShape",
    "ChEmitterAsset",
    "ChEnumMapperBase",
    "ChEnumMapper",
    "ChException",
    "ChExceptionArchive",
    "ChFEAContainer",
    "ChFile_ps",
    "ChFile_ps_axis_setting",
    "ChFile_ps_color",
    "ChFile_ps_graph_setting",
    "ChFileutils",
    "ChFluidContainer",
    "ChForce",
    "ChFrame",
    "ChFrameMoving",
    "ChFseqNode",
    "ChFunction",
    "ChFunction_Const",
    "ChFunction_ConstAcc",
    "ChFunction_Derive",
    "ChFunction_Fillet3",
    "ChFunction_Integrate",
    "ChFunction_Lambda",
    "ChFunction_Matlab",
    "ChFunction_Mirror",
    "ChFunction_Mocap",
    "ChFunction_Noise",
    "ChFunction_Operation",
    "ChFunction_Oscilloscope",
    "ChFunction_Poly",
    "ChFunction_Poly345",
    "ChFunction_Ramp",
    "ChFunction_Recorder",
    "ChFunction_Repeat",
    "ChFunction_Sequence",
    "ChFunction_Sigma",
    "ChFunction_Sine",
    "ChFunctorArchiveIn",
    "ChFunctorArchiveOut",
    "ChFx",
    "ChFxCfunction",
    "ChFxCfunctionS",
    "ChGenericConstraint",
    "ChGenericConstraint_Chf",
    "ChGenericConstraint_Chf_Continuity",
    "ChGenericConstraint_Chf_HorDistance",
    "ChGenericConstraint_Chf_ImposeVal",
    "ChGenericConstraint_Chf_VertDistance",
    "ChGenotype",
    "ChGlyphs",
    "ChImplicitIterativeTimestepper",
    "ChImplicitTimestepper",
    "ChIndexedNodes",
    "ChIndexedParticles",
    "ChInertiaUtils",
    "ChIntegrable",
    "ChIntegrable1D",
    "ChIntegrable2D",
    "ChIntegrable3D",
    "ChIntegrableIIorder",
    "ChIterativeSolver",
    "ChIterativeSolverParallel",
    "ChIterativeSolverParallelNSC",
    "ChIterativeSolverParallelSMC",
    "ChKblock",
    "ChKblockGeneric",
    "ChLineShape",
    "ChLink",
    "ChLinkBase",
    "ChLinkBrake",
    "ChLinkBushing",
    "ChLinkClearance",
    "ChLinkDistance",
    "ChLinkedListMatrix",
    "ChLinkEngine",
    "ChLinkForce",
    "ChLinkGear",
    "ChLinkLimit",
    "ChLinkLinActuator",
    "ChLinkLock",
    "ChLinkLockAlign",
    "ChLinkLockCylindrical",
    "ChLinkLockFree",
    "ChLinkLockLock",
    "ChLinkLockOldham",
    "ChLinkLockParallel",
    "ChLinkLockPerpend",
    "ChLinkLockPlanePlane",
    "ChLinkLockPointLine",
    "ChLinkLockPointPlane",
    "ChLinkLockPrismatic",
    "ChLinkLockRevolute",
    "ChLinkLockRevolutePrismatic",
    "ChLinkLockSpherical",
    "ChLinkMarkers",
    "ChLinkMask",
    "ChLinkMasked",
    "ChLinkMaskLF",
    "ChLinkMate",
    "ChLinkMateCoaxial",
    "ChLinkMateFix",
    "ChLinkMateGeneric",
    "ChLinkMateOrthogonal",
    "ChLinkMateParallel",
    "ChLinkMatePlane",
    "ChLinkMateSpherical",
    "ChLinkMateXdistance",
    "ChLinkPointSpline",
    "ChLinkPulley",
    "ChLinkRackpinion",
    "ChLinkRevolute",
    "ChLinkRevoluteSpherical",
    "ChLinkRevoluteTranslational",
    "ChLinkRotSpringCB",
    "ChLinkScrew",
    "ChLinkSpring",
    "ChLinkSpringCB",
    "ChLinkTrajectory",
    "ChLinkUniversal",
    "ChList",
    "ChLoad",
    "ChLoadable",
    "ChLoadableU",
    "ChLoadableUV",
    "ChLoadableUVW",
    "ChLoadBase",
    "ChLoadBodyBody",
    "ChLoadBodyBodyBushingGeneric",
    "ChLoadBodyBodyBushingMate",
    "ChLoadBodyBodyBushingPlastic",
    "ChLoadBodyBodyBushingSpherical",
    "ChLoadBodyForce",
    "ChLoadBodyMesh",
    "ChLoadBodyTorque",
    "ChLoadContainer",
    "ChLoadCustom",
    "ChLoadCustomMultiple",
    "ChLoader",
    "ChLoaderForceOnSurface",
    "ChLoaderGravity",
    "ChLoaderPressure",
    "ChLoaderU",
    "ChLoaderUatomic",
    "ChLoaderUdistributed",
    "ChLoaderUV",
    "ChLoaderUVatomic",
    "ChLoaderUVdistributed",
    "ChLoaderUVW",
    "ChLoaderUVWatomic",
    "ChLoaderUVWdistributed",
    "ChLoaderXYZnode",
    "ChLoadJacobians",
    "ChLoadXYZnode",
    "ChLog",
    "ChLogConsole",
    "ChMapMatrix",
    "ChMarker",
    "ChMaterialComposite",
    "ChMaterialCompositeNSC",
    "ChMaterialCompositeSMC",
    "ChMaterialCompositionStrategy",
    "ChMaterialSurface",
    "ChMaterialSurfaceNSC",
    "ChMaterialSurfaceSMC",
    "ChMatlabEngine",
    "ChMatrix",
    "ChMatrix33",
    "ChMatrixDynamic",
    "ChMatrixNM",
    "ChMatterSPH",
    "ChMelement",
    "ChMinMaxDistribution",
    "ChMklEngine",
    "ChNameValue",
    "ChNode",
    "ChNodeBase",
    "ChNodeSPH",
    "ChNodeXYZ",
    "ChNonlinearSolver",
    "ChNormalDistribution",
    "ChObj",
    "ChObjShapeFile",
    "ChOptimizer",
    "ChOptimizerGenetic",
    "ChOptimizerGradient",
    "ChOptimizerHybrid",
    "ChOptimizerLocal",
    "ChParallelDataManager",
    "ChParticleBase",
    "ChParticleContainer",
    "ChParticlesClones",
    "ChPathShape",
    "ChPhysicsItem",
    "ChPointPointDrawing",
    "ChPointPointSegment",
    "ChPointPointSpring",
    "ChProbe",
    "ChProjectConstraints",
    "ChProjectNone",
    "ChProximityContainer",
    "ChProximityContainerMeshless",
    "ChProximityContainerSPH",
    "ChProximityMeshless",
    "ChProximitySPH",
    "ChPythonEngine",
    "ChQuadrature",
    "ChQuadratureTables",
    "ChQuadratureTablesTetrahedron",
    "ChQuadratureTablesTriangle",
    "ChQuaternion",
    "ChRealtimeStepTimer",
    "ChRef",
    "ChRefFunction",
    "ChRefFunctionHandle",
    "ChRoundedBoxShape",
    "ChRoundedConeShape",
    "ChRoundedCylinderShape",
    "ChShaft",
    "ChShaftsBody",
    "ChShaftsClutch",
    "ChShaftsCouple",
    "ChShaftsGear",
    "ChShaftsGearbox",
    "ChShaftsGearboxAngled",
    "ChShaftsMotor",
    "ChShaftsPlanetary",
    "ChShaftsThermalEngine",
    "ChShaftsTorque",
    "ChShaftsTorqueBase",
    "ChShaftsTorqueConverter",
    "ChShaftsTorsionSpring",
    "ChSharedMassBody",
    "ChShurProduct",
    "ChShurProductBilateral",
    "ChShurProductFEM",
    "ChSolver",
    "ChSolverAPGD",
    "ChSolverBB",
    "ChSolverJacobi",
    "ChSolverMatlab",
    "ChSolverMINRES",
    "ChSolverMKL",
    "ChSolverParallel",
    "ChSolverParallelAPGD",
    "ChSolverParallelAPGDREF",
    "ChSolverParallelBB",
    "ChSolverParallelCG",
    "ChSolverParallelGS",
    "ChSolverParallelJacobi",
    "ChSolverParallelMinRes",
    "ChSolverParallelSPGQP",
    "ChSolverPCG",
    "ChSolverPMINRES",
    "ChSolverSMC",
    "ChSolverSOR",
    "ChSolverSORmultithread",
    "ChSolverSymmSOR",
    "ChSparseMatrix",
    "ChSparsityPatternLearner",
    "ChSphereShape",
    "ChState",
    "ChStateDelta",
    "ChStaticAnalysis",
    "ChStaticLinearAnalysis",
    "ChStaticNonLinearAnalysis",
    "ChStream",
    "ChStreamFile",
    "ChStreamIn",
    "ChStreamInAscii",
    "ChStreamInAsciiFile",
    "ChStreamInAsciiVector",
    "ChStreamInBinary",
    "ChStreamInBinaryFile",
    "ChStreamInBinaryStream",
    "ChStreamInBinaryVector",
    "ChStreamIstreamWrapper",
    "ChStreamOstreamWrapper",
    "ChStreamOut",
    "ChStreamOutAscii",
    "ChStreamOutAsciiFile",
    "ChStreamOutAsciiVector",
    "ChStreamOutBinary",
    "ChStreamOutBinaryFile",
    "ChStreamOutBinaryStream",
    "ChStreamOutBinaryVector",
    "ChStreamVectorWrapper",
    "ChSurfaceShape",
    "ChSystem",
    "ChSystemDescriptor",
    "ChSystemDescriptorParallel",
    "ChSystemNSC",
    "ChSystemParallel",
    "ChSystemParallelNSC",
    "ChSystemParallelSMC",
    "ChSystemSMC",
    "ChTexture",
    "ChTimer",
    "ChTimestepper",
    "ChTimestepperEulerExpl",
    "ChTimestepperEulerExplIIorder",
    "ChTimestepperEulerImplicit",
    "ChTimestepperEulerImplicitLinearized",
    "ChTimestepperEulerImplicitProjected",
    "ChTimestepperEulerSemiImplicit",
    "ChTimestepperHeun",
    "ChTimestepperHHT",
    "ChTimestepperIIorder",
    "ChTimestepperIorder",
    "ChTimestepperLeapfrog",
    "ChTimestepperNewmark",
    "ChTimestepperRungeKuttaExpl",
    "ChTimestepperTrapezoidal",
    "ChTimestepperTrapezoidalLinearized",
    "ChTimestepperTrapezoidalLinearized2",
    "ChTransform",
    "ChTriangleMeshShape",
    "ChVariables",
    "ChVariablesBody",
    "ChVariablesBodyOwnMass",
    "ChVariablesBodySharedMass",
    "ChVariablesGeneric",
    "ChVariablesGenericDiagonalMass",
    "ChVariablesNode",
    "ChVariablesShaft",
    "ChVariableTupleCarrier_1vars",
    "ChVector",
    "ChVector2",
    "ChVectorDynamic",
    "ChVisualization",
    "ChWeibullDistribution",
    "ChZhangDistribution",
];