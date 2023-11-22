//! This module contains functionality relevant to UDK logging.
use crate::dll::get_udk_slice;

/// Offset from the beginning of UDK64.exe to the debug log object.
#[cfg(target_arch = "x86_64")]
const DEBUG_LOG_OFFSET: usize = 0x0355_1720;
/// Address of UDK's log function.
#[cfg(target_arch = "x86_64")]
const DEBUG_FN_OFFSET: usize = 0x0024_6A20;

/// Offset from the beginning of UDK64.exe to the debug log object.
#[cfg(target_arch = "x86")]
const DEBUG_LOG_OFFSET: usize = 0x029a_31a8;
/// Address of UDK's log function.
#[cfg(target_arch = "x86")]
const DEBUG_FN_OFFSET: usize = 0x0002_1c500;

/// This is the type signature of UDK's log function.
type UDKLogFn = unsafe extern "C" fn(usize, u32, *const widestring::WideChar);

/// This enum represents the UDK message types.
#[repr(u32)]
pub enum LogType {
    //Exporter = 0x9f,
    //StackNode = 0xa0,
    //Property = 0xa1,
    //Camera = 0xa2,
    //PlayerInput = 0xa3,
    //Actor = 0xa4,
    //ObjectRedirector = 0xa5,
    //ObjectArchetype = 0xa6,
    //Vect = 0x258,
    //Rot = 0x259,
    //ArrayCount = 0x25d,
    //EnumCount = 0x25e,
    //Rng = 0x25f,
    //NameOf = 0x260,
    //Else = 0x26c,
    //If = 0x26d,
    //Goto = 0x26e,
    //Stop = 0x26f,
    //Until = 0x271,
    //While = 0x272,
    //Do = 0x273,
    //Break = 0x274,
    //For = 0x275,
    //ForEach = 0x276,
    //Assert = 0x277,
    //Switch = 0x278,
    //Case = 0x279,
    //Default = 0x27a,
    //Continue = 0x27b,
    //ElseIf = 0x27c,
    //FilterEditorOnly = 0x27d,
    //Private = 0x280,
    //Const = 0x281,
    //Out = 0x282,
    //Export = 0x283,
    //DuplicateTransient = 0x284,
    //NoImport = 0x285,
    //Skip = 0x286,
    //Coerce = 0x287,
    //Optional = 0x288,
    //Config = 0x28a,
    //EditorOnly = 0x28b,
    //NotForConsole = 0x28c,
    //EditConst = 0x28d,
    //Localized = 0x28e,
    //GlobalConfig = 0x28f,
    //SafeReplace = 0x290,
    //New = 0x291,
    //Protected = 0x292,
    //Public = 0x293,
    //EditInline = 0x294,
    //EditInlineUse = 0x295,
    //Deprecated = 0x296,
    //Atomic = 0x297,
    //Immutable = 0x298,
    //Automated = 0x299,
    //RepNotify = 0x29a,
    //Interp = 0x29b,
    //NoClear = 0x29c,
    //NonTransactional = 0x29d,
    //EditFixedSize = 0x29e,
    //ImmutableWhenCooked = 0x3ac,
    //RepRetry = 0x3ad,
    //PrivateWrite = 0x3ae,
    //ProtectedWrite = 0x3af,
    //EditHide = 0x3b0,
    //EditTextBox = 0x3b1,
    //Intrinsic = 0x29f,
    //Within = 0x2a0,
    //Abstract = 0x2a1,
    //Package = 0x2a2,
    //Guid = 0x2a3,
    //Parent = 0x2a4,
    //Class = 0x2a5,
    //Extends = 0x2a6,
    //NoExport = 0x2a7,
    //Placeable = 0x2a8,
    //PerObjectConfig = 0x2a9,
    //NativeReplication = 0x2aa,
    //NotPlaceable = 0x2ab,
    //EditInlineNew = 0x2ac,
    //NotEditInlineNew = 0x2ad,
    //HideCategories = 0x2ae,
    //ShowCategories = 0x2af,
    //CollapseCategories = 0x2b0,
    //DontCollapseCategories = 0x2b1,
    //DependsOn = 0x2ba,
    //HideDropDown = 0x2bb,
    //Implements = 0x3b6,
    //Interface = 0x3b7,
    //Inherits = 0x3b8,
    //PerObjectLocalized = 0x3b9,
    //NonTransient = 0x3ba,
    //Archetype = 0x3bb,
    //StrictConfig = 0x3bc,
    //UnusedStructKeyword1 = 0x3bd,
    //UnusedStructKeyword2 = 0x3be,
    //SerializeText = 0x3bf,
    //CrossLevel = 0x3c0,
    //CrossLevelActive = 0x3c1,
    //CrossLevelPassive = 0x3c2,
    //ClassGroup = 0x3c3,
    //Auto = 0x2b2,
    //Ignores = 0x2b3,
    //Instanced = 0x2b4,
    //Component = 0x2b5,
    //Components = 0x2b6,
    //Global = 0x2b7,
    //Super = 0x2b8,
    //Outer = 0x2b9,
    //Operator = 0x2bc,
    //PreOperator = 0x2bd,
    //PostOperator = 0x2be,
    //Final = 0x2bf,
    //Iterator = 0x2c0,
    //Latent = 0x2c1,
    //Return = 0x2c2,
    //Singular = 0x2c3,
    //Simulated = 0x2c4,
    //Exec = 0x2c5,
    //Event = 0x2c6,
    //Static = 0x2c7,
    //Native = 0x2c8,
    //Invariant = 0x2c9,
    //Delegate = 0x2ca,
    //Virtual = 0x2cb,
    //NoExportHeader = 0x2cc,
    //DLLImport = 0x2cd,
    //NativeOnly = 0x2ce,
    //UnusedFunctionKeyword3 = 0x2cf,
    //Var = 0x2d0,
    //Local = 0x2d1,
    //Import = 0x2d2,
    //From = 0x2d3,
    //Spawn = 0x2da,
    //Array = 0x2db,
    //Map = 0x2dc,
    //AutoExpandCategories = 0x2dd,
    //AutoCollapseCategories = 0x2de,
    //DontAutoCollapseCategories = 0x2df,
    //DontSortCategories = 0x2e0,
    //Tag = 0x2e4,
    //Role = 0x2e6,
    //RemoteRole = 0x2e7,
    //System = 0x2e8,
    //User = 0x2e9,
    //PersistentLevel = 0x2ea,
    //TheWorld = 0x2eb,
    //Benchmark = 0x2ec,
    //Windows = 0x2ee,
    //XBox = 0x2ef,
    //PlayStation = 0x2f0,
    //Linux = 0x2f1,
    //MacOSX = 0x2f2,
    //Pc = 0x2f3,
    //PerfWarning = 0x2f5,
    //DevLive = 0x2f7,
    Log = 0x2f8,
    //Critical = 0x2f9,
    Init = 0x2fa,
    //Exit = 0x2fb,
    //Cmd = 0x2fc,
    //Play = 0x2fd,
    //Console = 0x2fe,
    //Warning = 0x2ff,
    //ExecWarning = 0x300,
    //ScriptWarning = 0x301,
    //ScriptLog = 0x302,
    //Compatibility = 0x312,
    //NetComeGo = 0x313,
    Error = 0x315,
    //Heading = 0x316,
    //SubHeading = 0x317,
    //FriendlyError = 0x318,
    //Progress = 0x319,
    //UserPrompt = 0x31a,
    //SourceControl = 0x31b,
    //DevPhysics = 0x31c,
    //DevMemory = 0x329,
    //Xma = 0x32a,
    //Wav = 0x32b,
    //AILog = 0x32c,
    //DevParticle = 0x32d,
    //PerfEvent = 0x32e,
    //ParticleWarn = 0x333,
    //UTrace = 0x356,
    //DevSpawn = 0x359,
    //Hack = 0x35b,
    //DevDataBase = 0x45f,
    //DevHddCaching = 0x466,
    //DevPatch = 0x46a,
    //DebugState = 0xcd,
    //White = 0x320,
    //Black = 0x321,
    //Red = 0x322,
    //Green = 0x323,
    //Blue = 0x324,
    //Cyan = 0x325,
    //Magenta = 0x326,
    //Yellow = 0x327,
    //DefaultColor = 0x328,
    //KeyType = 0x334,
    //KeyEvent = 0x335,
    //Write = 0x336,
    //Message = 0x337,
    //InitialState = 0x338,
    //Texture = 0x339,
    //Sound = 0x33a,
    //FireTexture = 0x33b,
    //IceTexture = 0x33c,
    //WaterTexture = 0x33d,
    //WaveTexture = 0x33e,
    //WetTexture = 0x33f,
    //Main = 0x340,
    //VideoChange = 0x342,
    //SendText = 0x343,
    //SendBinary = 0x344,
    //ConnectFailure = 0x345,
    //Length = 0x346,
    //Insert = 0x347,
    //Remove = 0x348,
    //Add = 0x4b0,
    //AddItem = 0x4b1,
    //RemoveItem = 0x4b2,
    //InsertItem = 0x4b3,
    //Sort = 0x4b4,
    //Game = 0x349,
    //SequenceObjects = 0x34a,
    //PausedState = 0x34b,
    //ContinuedState = 0x34c,
    //SelectionColor = 0x34d,
    //Find = 0x34e,
    //UI = 0x34f,
    //DataBinding = 0x352,
    //OptionMusic = 0x353,
    //OptionSFX = 0x354,
    //OptionVoice = 0x355,
    //Linear = 0x462,
    //Point = 0x463,
    //Aniso = 0x464,
    //Master = 0x35c,
    //Ambient = 0x35d,
    //UnGrouped = 0x35e,
    //VoiceChat = 0x35f,
    //Brush = 0x4b8,
    //Attributes = 0x361,
    //Strings = 0x362,
    //Images = 0x363,
    //SceneData = 0x364,
    //EndIf = 0x366,
    //Include = 0x367,
    //Define = 0x368,
    //Undefine = 0x369,
    //IsDefined = 0x36a,
    //NotDefined = 0x36b,
    //Debug = 0x36c,
    //Counter = 0x36d,
    //SetCounter = 0x36e,
    //GetCounter = 0x36f,
    //EngineVersion = 0x370,
    //IfCondition = 0x371,
    //FontCharacter = 0x384,
    //SourceState = 0x385,
    //InitChild2StartBone = 0x386,
    //SourceStyle = 0x387,
    //SoundCueLocalized = 0x388,
    //SoundCue = 0x389,
    //RawDistributionFloat = 0x38a,
    //RawDistributionVector = 0x38b,
    //UIDockingSet = 0x38c,
    //DockPadding = 0x38d,
    //ScaleType = 0x390,
    //EvalType = 0x391,
    //AutoSizePadding = 0x392,
    //PlayerWalking = 0x393,
    //PlayerClimbing = 0x394,
    //PlayerDriving = 0x395,
    //PlayerSwimming = 0x396,
    //PlayerFlying = 0x397,
    //Spectating = 0x398,
    //PlayerWaiting = 0x399,
    //WaitingForPawn = 0x39a,
    //RoundEnded = 0x39b,
    //Dead = 0x39c,
    //GearGeneral = 0x3e8,
    //GearActiveReload = 0x3e9,
    //GearMiniGames = 0x3ea,
    //GearResurrectionSystem = 0x3eb,
    //GearVehicleSystem = 0x3ec,
    //GearCheckpointSystem = 0x3ed,
    //Cover = 0x3ee,
    //AICommand = 0x3ef,
    //Success = 0x3f0,
    //Failure = 0x3f1,
    //Aborted = 0x3f2,
    //PlayerTakingCover = 0x3f3,
    //Engaging = 0x3f4,
    //PlayerTurreting = 0x3f5,
    //Reviving = 0x3f6,
    //PlayerID = 0x438,
    //TeamID = 0x439,
    //HearSoundFinished = 0x43a,
    //OnParticleSystemFinished = 0x43b,
    //Time = 0x44c,
    //PPVolume_BloomEffect = 0x44d,
    //PPVolume_DOFEffect = 0x44e,
    //PPVolume_MotionBlurEffect = 0x44f,
    //PPVolume_SceneEffect = 0x450,
    //PPVolume_DOFAndBloomEffect = 0x451,
    //Desaturation = 0x452,
    //HighLights = 0x453,
    //MidTones = 0x454,
    //Shadows = 0x455,
    //PPVolume_UberPostProcessEffect = 0x456,
    //DeviceID = 0x457,
    //InterpCurveFloat = 0x458,
    //InterpCurveVector2D = 0x459,
    //InterpCurveVector = 0x45a,
    //InterpCurveTwoVectors = 0x45b,
    //InterpCurveQuat = 0x45c,
    //UniqueNetId = 0x45d,
    //PopUp = 0x46d,
    //Team = 0x46e,
    //DevDlc = 0x46f,
    //Landscape_RedTexture = 0x474,
    //Landscape_GreenTexture = 0x475,
    //Landscape_BlueTexture = 0x476,
    //Landscape_RedMask = 0x477,
    //Landscape_GreenMask = 0x478,
    //Landscape_BlueMask = 0x479,
    //Base = 0x47e,
    //Specular = 0x47f,
    //Emissive = 0x480,
    //Environment = 0x481,
    //RimLighting = 0x482,
    //BumpOffset = 0x483,
    //Masking = 0x484,
    //TextureBlending = 0x485,
    //ColorBlending = 0x486,
    //TextureTransform = 0x487,
    //VertexAnimation = 0x488,
    //Diffuse = 0x489,
    //Normal = 0x48a,
    //MobileSpecularPower = 0x48d,
    //MobileEnvironmentAmount = 0x48e,
    //MobileEnvironmentFresnelAmount = 0x48f,
    //MobileEnvironmentFresnelExponent = 0x490,
    //MobileRimLightingStrength = 0x491,
    //MobileRimLightingExponent = 0x492,
    //MobileBumpOffsetReferencePlane = 0x493,
    //MobileBumpOffsetHeightRatio = 0x494,
    //MobileTransformCenterX = 0x495,
    //MobileTransformCenterY = 0x496,
    //MobilePannerSpeedX = 0x497,
    //MobilePannerSpeedY = 0x498,
    //MobileRotateSpeed = 0x499,
    //MobileFixedScaleX = 0x49a,
    //MobileFixedScaleY = 0x49b,
    //MobileSineScaleX = 0x49c,
    //MobileSineScaleY = 0x49d,
    //MobileSineScaleFrequencyMultipler = 0x49e,
    //MobileFixedOffsetX = 0x49f,
    //MobileFixedOffsetY = 0x4a0,
    //MobileTangentVertexFrequencyMultiplier = 0x4a1,
    //MobileVerticalFrequencyMultiplier = 0x4a2,
    //MobileMaxVertexMovementAmplitude = 0x4a3,
    //MobileSwayFrequencyMultiplier = 0x4a4,
    //MobileSwayMaxAngle = 0x4a5,
    //MobileSpecularColor = 0x4a6,
    //MobileEmissiveColor = 0x4a7,
    //MobileEnvironmentColor = 0x4a8,
    //MobileRimLightingColor = 0x4a9,
    //MobileDefaultUniformColor = 0x4aa,
    //MobileOpacityMultiplier = 0x4ab,
    //MobileBaseTexture = 0x4ce,
    //MobileNormalTexture = 0x4cf,
    //MobileEmissiveTexture = 0x4d0,
    //MobileMaskTexture = 0x4d1,
    //MobileDetailTexture = 0x4d2,
    //MobileDetailTexture2 = 0x4d3,
    //MobileDetailTexture3 = 0x4d4,
    //MobileEnvironmentTexture = 0x4d5,
    //FortressMCP = 0x420,
    //Destroyed = 0x12c,
    //GainedChild = 0x12d,
    //LostChild = 0x12e,
    //HitWall = 0x12f,
    //Falling = 0x130,
    //Landed = 0x131,
    //Touch = 0x132,
    //UnTouch = 0x133,
    //Bump = 0x134,
    //BeginState = 0x135,
    //EndState = 0x136,
    //BaseChange = 0x137,
    //Attach = 0x138,
    //Detach = 0x139,
    //EncroachingOn = 0x13a,
    //EncroachedBy = 0x13b,
    //MayFall = 0x13c,
    //Tick = 0x13d,
    //SeePlayer = 0x13e,
    //EnemyNotVisible = 0x13f,
    //HearNoise = 0x140,
    //UpdateEyeHeight = 0x141,
    //SeeMonster = 0x142,
    //SpecialHandling = 0x143,
    //BotDesireability = 0x144,
    //NotifyBump = 0x145,
    //NotifyLanded = 0x146,
    //NotifyHitWall = 0x147,
    //PreBeginPlay = 0x148,
    //PostBeginPlay = 0x149,
    //UnusedProbe = 0x14a,
    //All = 0x14b,
    //PoppedState = 0x18e,
    //PushedState = 0x18f,
    //MeshEmitterVertexColor = 0x190,
    //TextureOffsetParameter = 0x191,
    //TextureScaleParameter = 0x192,
    //ImpactVel = 0x193,
    //SlideVel = 0x194,
    //TextureOffset1Parameter = 0x195,
    //MeshEmitterDynamicParameter = 0x196,
    //ExpressionInput = 0x197,
    //OnAudioFinished = 0x4b5,
    //ForceScriptOrder = 0x4b6,
    //Mobile = 0x4b7,
    //Untitled = 0x4b9,
    //Timer = 0x4ba,
    //PS3 = 0x4bb,
}

/// Log a message via the UDK logging framework.
pub fn log(typ: LogType, msg: &str) {
    let udk_slice = get_udk_slice();
    let log_obj = unsafe { udk_slice.as_ptr().add(DEBUG_LOG_OFFSET) };
    let log_fn: UDKLogFn = unsafe { std::mem::transmute(udk_slice.as_ptr().add(DEBUG_FN_OFFSET)) };

    // Convert the UTF-8 Rust string into an OS wide string.
    let wmsg: widestring::U16CString = widestring::WideCString::from_str(format!("discord.dll: {}", msg)).unwrap();

    unsafe {
        (log_fn)(log_obj as usize, typ as u32, wmsg.as_ptr());
    }
}