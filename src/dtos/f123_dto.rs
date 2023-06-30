use serde::Deserialize;

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketHeader {
    pub m_packetFormat: u16,           // 2023
    pub m_gameYear: u8,                // Game year - last two digits e.g. 23
    pub m_gameMajorVersion: u8,        // Game major version - "X.00"
    pub m_gameMinorVersion: u8,        // Game minor version - "1.XX"
    pub m_packetVersion: u8,           // Version of this packet type, all start from 1
    pub m_packetId: u8,                // Identifier for the packet type, see below
    pub m_sessionUID: u64,             // Unique identifier for the session
    pub m_sessionTime: f32,            // Session timestamp
    pub m_frameIdentifier: u32,        // Identifier for the frame the data was retrieved on
    pub m_overallFrameIdentifier: u32, // Overall identifier for the frame the data was retrieved  // on, doesn't go back after flashbacks
    pub m_playerCarIndex: u8,          // Index of player's car in the array
    pub m_secondaryPlayerCarIndex: u8, // Index of secondary player's car in the array (splitscreen) // 255 if no second player
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CarMotionData {
    pub m_worldPositionX: f32,     // World space X position - metres
    pub m_worldPositionY: f32,     // World space Y position
    pub m_worldPositionZ: f32,     // World space Z position
    pub m_worldVelocityX: f32,     // Velocity in world space X – metres/s
    pub m_worldVelocityY: f32,     // Velocity in world space Y
    pub m_worldVelocityZ: f32,     // Velocity in world space Z
    pub m_worldForwardDirX: i16,   // World space forward X direction (normalized)
    pub m_worldForwardDirY: i16,   // World space forward Y direction (normalized)
    pub m_worldForwardDirZ: i16,   // World space forward Z direction (normalized)
    pub m_worldRightDirX: i16,     // World space right X direction (normalized)
    pub m_worldRightDirY: i16,     // World space right Y direction (normalized)
    pub m_worldRightDirZ: i16,     // World space right Z direction (normalized)
    pub m_gForceLateral: f32,      // Lateral G-Force component
    pub m_gForceLongitudinal: f32, // Longitudinal G-Force component
    pub m_gForceVertical: f32,     // Vertical G-Force component
    pub m_yaw: f32,                // Yaw angle in radians
    pub m_pitch: f32,              // Pitch angle in radians
    pub m_roll: f32,               // Roll angle in radians
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketMotionData {
    pub m_header: PacketHeader,               // Header
    pub m_carMotionData: [CarMotionData; 22], // Data for all cars on track
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct MarshalZone {
    pub m_zoneStart: f32, // Fraction (0..1) of way through the lap the marshal zone starts
    pub m_zoneFlag: i8,   // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct WeatherForecastSample {
    pub m_sessionType: u8, // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P, 5 = Q1
    // 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ, 10 = R, 11 = R2
    // 12 = R3, 13 = Time Trial
    pub m_timeOffset: u8, // Time in minutes the forecast is for
    pub m_weather: u8,    // Weather - 0 = clear, 1 = light cloud, 2 = overcast
    // 3 = light rain, 4 = heavy rain, 5 = storm
    pub m_trackTemperature: i8,       // Track temp. in degrees Celsius
    pub m_trackTemperatureChange: i8, // Track temp. change – 0 = up, 1 = down, 2 = no change
    pub m_airTemperature: i8,         // Air temp. in degrees Celsius
    pub m_airTemperatureChange: i8,   // Air temp. change – 0 = up, 1 = down, 2 = no change
    pub m_rainPercentage: u8,         // Rain percentage (0-100)
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketSessionData {
    pub m_header: PacketHeader, // Header
    pub m_weather: u8,          // Weather - 0 = clear, 1 = light cloud, 2 = overcast
    // 3 = light rain, 4 = heavy rain, 5 = storm
    pub m_trackTemperature: i8, // Track temp. in degrees Celsius
    pub m_airTemperature: i8,   // Air temp. in degrees Celsius
    pub m_totalLaps: u8,        // Total number of laps in this race
    pub m_trackLength: u16,     // Track length in metres
    pub m_sessionType: u8,      // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
    // 5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
    // 10 = R, 11 = R2, 12 = R3, 13 = Time Trial
    pub m_trackId: i8, // -1 for unknown, see appendix
    pub m_formula: u8, // Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
    // 3 = F1 Generic, 4 = Beta, 5 = Supercars
    // 6 = Esports, 7 = F2 2021
    pub m_sessionTimeLeft: u16,    // Time left in session in seconds
    pub m_sessionDuration: u16,    // Session duration in seconds
    pub m_pitSpeedLimit: u8,       // Pit speed limit in kilometres per hour
    pub m_gamePaused: u8,          // Whether the game is paused – network game only
    pub m_isSpectating: u8,        // Whether the player is spectating
    pub m_spectatorCarIndex: u8,   // Index of the car being spectated
    pub m_sliProNativeSupport: u8, // SLI Pro support, 0 = inactive, 1 = active
    pub m_numMarshalZones: u8,     // Number of marshal zones to follow
    pub m_marshalZones: [MarshalZone; 21], // List of marshal zones – max 21
    pub m_safetyCarStatus: u8,     // 0 = no safety car, 1 = full
    // 2 = virtual, 3 = formation lap
    pub m_networkGame: u8,               // 0 = offline, 1 = online
    pub m_numWeatherForecastSamples: u8, // Number of weather samples to follow
    pub m_weatherForecastSamples: Vec<WeatherForecastSample>, // Array of weather forecast samples
    pub m_forecastAccuracy: u8,          // 0 = Perfect, 1 = Approximate
    pub m_aiDifficulty: u8,              // AI Difficulty rating – 0-110
    pub m_seasonLinkIdentifier: u32,     // Identifier for season - persists across saves
    pub m_weekendLinkIdentifier: u32,    // Identifier for weekend - persists across saves
    pub m_sessionLinkIdentifier: u32,    // Identifier for session - persists across saves
    pub m_pitStopWindowIdealLap: u8,     // Ideal lap to pit on for current strategy (player)
    pub m_pitStopWindowLatestLap: u8,    // Latest lap to pit on for current strategy (player)
    pub m_pitStopRejoinPosition: u8,     // Predicted position to rejoin at (player)
    pub m_steeringAssist: u8,            // 0 = off, 1 = on
    pub m_brakingAssist: u8,             // 0 = off, 1 = low, 2 = medium, 3 = high
    pub m_gearboxAssist: u8,             // 1 = manual, 2 = manual & suggested gear, 3 = auto
    pub m_pitAssist: u8,                 // 0 = off, 1 = on
    pub m_pitReleaseAssist: u8,          // 0 = off, 1 = on
    pub m_ERSAssist: u8,                 // 0 = off, 1 = on
    pub m_DRSAssist: u8,                 // 0 = off, 1 = on
    pub m_dynamicRacingLine: u8,         // 0 = off, 1 = corners only, 2 = full
    pub m_dynamicRacingLineType: u8,     // 0 = 2D, 1 = 3D
    pub m_gameMode: u8,                  // Game mode id - see appendix
    pub m_ruleSet: u8,                   // Ruleset - see appendix
    pub m_timeOfDay: u32,                // Local time of day - minutes since midnight
    pub m_sessionLength: u8,             // 0 = None, 2 = Very Short, 3 = Short, 4 = Medium
    // 5 = Medium Long, 6 = Long, 7 = Full
    pub m_speedUnitsLeadPlayer: u8,            // 0 = MPH, 1 = KPH
    pub m_temperatureUnitsLeadPlayer: u8,      // 0 = Celsius, 1 = Fahrenheit
    pub m_speedUnitsSecondaryPlayer: u8,       // 0 = MPH, 1 = KPH
    pub m_temperatureUnitsSecondaryPlayer: u8, // 0 = Celsius, 1 = Fahrenheit
    pub m_numSafetyCarPeriods: u8,             // Number of safety cars called during session
    pub m_numVirtualSafetyCarPeriods: u8,      // Number of virtual safety cars called
    pub m_numRedFlagPeriods: u8,               // Number of red flags called during session
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct LapData {
    pub m_lastLapTimeInMS: u32,            // Last lap time in milliseconds
    pub m_currentLapTimeInMS: u32,         // Current time around the lap in milliseconds
    pub m_sector1TimeInMS: u16,            // Sector 1 time in milliseconds
    pub m_sector1TimeMinutes: u8,          // Sector 1 whole minute part
    pub m_sector2TimeInMS: u16,            // Sector 2 time in milliseconds
    pub m_sector2TimeMinutes: u8,          // Sector 2 whole minute part
    pub m_deltaToCarInFrontInMS: u16,      // Time delta to car in front in milliseconds
    pub m_deltaToRaceLeaderInMS: u16,      // Time delta to race leader in milliseconds
    pub m_lapDistance: f32, // Distance vehicle is around current lap in metres – could be negative if line hasn’t been crossed yet
    pub m_totalDistance: f32, // Total distance travelled in session in metres – could be negative if line hasn’t been crossed yet
    pub m_safetyCarDelta: f32, // Delta in seconds for safety car
    pub m_carPosition: u8,    // Car race position
    pub m_currentLapNum: u8,  // Current lap number
    pub m_pitStatus: u8,      // 0 = none, 1 = pitting, 2 = in pit area
    pub m_numPitStops: u8,    // Number of pit stops taken in this race
    pub m_sector: u8,         // 0 = sector1, 1 = sector2, 2 = sector3
    pub m_currentLapInvalid: u8, // Current lap invalid - 0 = valid, 1 = invalid
    pub m_penalties: u8,      // Accumulated time penalties in seconds to be added
    pub m_totalWarnings: u8,  // Accumulated number of warnings issued
    pub m_cornerCuttingWarnings: u8, // Accumulated number of corner cutting warnings issued
    pub m_numUnservedDriveThroughPens: u8, // Num drive through pens left to serve
    pub m_numUnservedStopGoPens: u8, // Num stop go pens left to serve
    pub m_gridPosition: u8,   // Grid position the vehicle started the race in
    pub m_driverStatus: u8, // Status of driver - 0 = in garage, 1 = flying lap, 2 = in lap, 3 = out lap, 4 = on track
    pub m_resultStatus: u8, // Result status - 0 = invalid, 1 = inactive, 2 = active, 3 = finished, 4 = didnotfinish, 5 = disqualified, 6 = not classified, 7 = retired
    pub m_pitLaneTimerActive: u8, // Pit lane timing, 0 = inactive, 1 = active
    pub m_pitLaneTimeInLaneInMS: u16, // If active, the current time spent in the pit lane in ms
    pub m_pitStopTimerInMS: u16, // Time of the actual pit stop in ms
    pub m_pitStopShouldServePen: u8, // Whether the car should serve a penalty at this stop
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketLapData {
    pub m_header: PacketHeader,     // Header
    pub m_lapData: [LapData; 22],   // Lap data for all cars on track
    pub m_timeTrialPBCarIdx: u8,    // Index of Personal Best car in time trial (255 if invalid)
    pub m_timeTrialRivalCarIdx: u8, // Index of Rival car in time trial (255 if invalid)
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub enum EventDataDetails {
    FastestLap {
        vehicleIdx: u8, // Vehicle index of car achieving fastest lap
        lapTime: f32,   // Lap time is in seconds
    },
    Retirement {
        vehicleIdx: u8, // Vehicle index of car retiring
    },
    TeamMateInPits {
        vehicleIdx: u8, // Vehicle index of team mate
    },
    RaceWinner {
        vehicleIdx: u8, // Vehicle index of the race winner
    },
    Penalty {
        penaltyType: u8,      // Penalty type – see Appendices
        infringementType: u8, // Infringement type – see Appendices
        vehicleIdx: u8,       // Vehicle index of the car the penalty is applied to
        otherVehicleIdx: u8,  // Vehicle index of the other car involved
        time: u8,             // Time gained, or time spent doing action in seconds
        lapNum: u8,           // Lap the penalty occurred on
        placesGained: u8,     // Number of places gained by this
    },
    SpeedTrap {
        vehicleIdx: u8,                 // Vehicle index of the vehicle triggering speed trap
        speed: f32,                     // Top speed achieved in kilometres per hour
        isOverallFastestInSession: u8,  // Overall fastest speed in session = 1, otherwise 0
        isDriverFastestInSession: u8,   // Fastest speed for driver in session = 1, otherwise 0
        fastestVehicleIdxInSession: u8, // Vehicle index of the vehicle that is the fastest in this session
        fastestSpeedInSession: f32,     // Speed of the vehicle that is the fastest in this session
    },
    StartLights {
        numLights: u8, // Number of lights showing
    },
    DriveThroughPenaltyServed {
        vehicleIdx: u8, // Vehicle index of the vehicle serving drive through
    },
    StopGoPenaltyServed {
        vehicleIdx: u8, // Vehicle index of the vehicle serving stop go
    },
    Flashback {
        flashbackFrameIdentifier: u32, // Frame identifier flashed back to
        flashbackSessionTime: f32,     // Session time flashed back to
    },
    Buttons {
        buttonStatus: u32, // Bit flags specifying which buttons are being pressed currently - see appendices
    },
    Overtake {
        overtakingVehicleIdx: u8,     // Vehicle index of the vehicle overtaking
        beingOvertakenVehicleIdx: u8, // Vehicle index of the vehicle being overtaken
    },
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketEventData {
    pub m_header: PacketHeader,           // Header
    pub m_eventStringCode: [u8; 4],       // Event string code, see below
    pub m_eventDetails: EventDataDetails, // Event details - should be interpreted differently for each type
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ParticipantData {
    pub m_aiControlled: u8, // Whether the vehicle is AI (1) or Human (0) controlled
    pub m_driverId: u8,     // Driver id - see appendix, 255 if network human
    pub m_networkId: u8,    // Network id – unique identifier for network players
    pub m_teamId: u8,       // Team id - see appendix
    pub m_myTeam: u8,       // My team flag – 1 = My Team, 0 = otherwise
    pub m_raceNumber: u8,   // Race number of the car
    pub m_nationality: u8,  // Nationality of the driver
    pub m_name: String,     // Name of participant in UTF-8 format – null terminated
    // Will be truncated with … (U+2026) if too long
    pub m_yourTelemetry: u8, // The player's UDP setting, 0 = restricted, 1 = public
    pub m_showOnlineNames: u8, // The player's show online names setting, 0 = off, 1 = on
    pub m_platform: u8,      // 1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketParticipantsData {
    pub m_header: PacketHeader, // Header
    pub m_numActiveCars: u8, // Number of active cars in the data – should match number of cars on HUD
    pub m_participants: [ParticipantData; 22],
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CarSetupData {
    pub m_frontWing: u8,               // Front wing aero
    pub m_rearWing: u8,                // Rear wing aero
    pub m_onThrottle: u8,              // Differential adjustment on throttle (percentage)
    pub m_offThrottle: u8,             // Differential adjustment off throttle (percentage)
    pub m_frontCamber: f32,            // Front camber angle (suspension geometry)
    pub m_rearCamber: f32,             // Rear camber angle (suspension geometry)
    pub m_frontToe: f32,               // Front toe angle (suspension geometry)
    pub m_rearToe: f32,                // Rear toe angle (suspension geometry)
    pub m_frontSuspension: u8,         // Front suspension
    pub m_rearSuspension: u8,          // Rear suspension
    pub m_frontAntiRollBar: u8,        // Front anti-roll bar
    pub m_rearAntiRollBar: u8,         // Front anti-roll bar
    pub m_frontSuspensionHeight: u8,   // Front ride height
    pub m_rearSuspensionHeight: u8,    // Rear ride height
    pub m_brakePressure: u8,           // Brake pressure (percentage)
    pub m_brakeBias: u8,               // Brake bias (percentage)
    pub m_rearLeftTyrePressure: f32,   // Rear left tyre pressure (PSI)
    pub m_rearRightTyrePressure: f32,  // Rear right tyre pressure (PSI)
    pub m_frontLeftTyrePressure: f32,  // Front left tyre pressure (PSI)
    pub m_frontRightTyrePressure: f32, // Front right tyre pressure (PSI)
    pub m_ballast: u8,                 // Ballast
    pub m_fuelLoad: f32,               // Fuel load
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketCarSetupData {
    pub m_header: PacketHeader, // Header
    pub m_carSetups: [CarSetupData; 22],
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CarTelemetryData {
    pub m_speed: u16,                       // Speed of car in kilometres per hour
    pub m_throttle: f32,                    // Amount of throttle applied (0.0 to 1.0)
    pub m_steer: f32, // Steering (-1.0 (full lock left) to 1.0 (full lock right))
    pub m_brake: f32, // Amount of brake applied (0.0 to 1.0)
    pub m_clutch: u8, // Amount of clutch applied (0 to 100)
    pub m_gear: i8,   // Gear selected (1-8, N=0, R=-1)
    pub m_engineRPM: u16, // Engine RPM
    pub m_drs: u8,    // 0 = off, 1 = on
    pub m_revLightsPercent: u8, // Rev lights indicator (percentage)
    pub m_revLightsBitValue: u16, // Rev lights (bit 0 = leftmost LED, bit 14 = rightmost LED)
    pub m_brakesTemperature: [u16; 4], // Brakes temperature (celsius)
    pub m_tyresSurfaceTemperature: [u8; 4], // Tyres surface temperature (celsius)
    pub m_tyresInnerTemperature: [u8; 4], // Tyres inner temperature (celsius)
    pub m_engineTemperature: u16, // Engine temperature (celsius)
    pub m_tyresPressure: [f32; 4], // Tyres pressure (PSI)
    pub m_surfaceType: [u8; 4], // Driving surface, see appendices
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketCarTelemetryData {
    pub m_header: PacketHeader, // Header
    pub m_carTelemetryData: [CarTelemetryData; 22],
    pub m_mfdPanelIndex: u8, // Index of MFD panel open - 255 = MFD closed
    // Single player, race – 0 = Car setup, 1 = Pits
    // 2 = Damage, 3 =  Engine, 4 = Temperatures
    // May vary depending on game mode
    pub m_mfdPanelIndexSecondaryPlayer: u8, // See above
    pub m_suggestedGear: i8,                // Suggested gear for the player (1-8)
                                            // 0 if no gear suggested
}
#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CarStatusData {
    pub m_tractionControl: u8, // Traction control - 0 = off, 1 = medium, 2 = full
    pub m_antiLockBrakes: u8,  // 0 (off) - 1 (on)
    pub m_fuelMix: u8,         // Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    pub m_frontBrakeBias: u8,  // Front brake bias (percentage)
    pub m_pitLimiterStatus: u8, // Pit limiter status - 0 = off, 1 = on
    pub m_fuelInTank: f32,     // Current fuel mass
    pub m_fuelCapacity: f32,   // Fuel capacity
    pub m_fuelRemainingLaps: f32, // Fuel remaining in terms of laps (value on MFD)
    pub m_maxRPM: u16,         // Cars max RPM, point of rev limiter
    pub m_idleRPM: u16,        // Cars idle RPM
    pub m_maxGears: u8,        // Maximum number of gears
    pub m_drsAllowed: u8,      // 0 = not allowed, 1 = allowed
    pub m_drsActivationDistance: u16, // 0 = DRS not available, non-zero - DRS will be available in [X] metres
    pub m_actualTyreCompound: u8,     // F1 Modern - 16 = C5, 17 = C4, 18 = C3, 19 = C2, 20 = C1
    // 21 = C0, 7 = inter, 8 = wet
    // F1 Classic - 9 = dry, 10 = wet
    // F2 – 11 = super soft, 12 = soft, 13 = medium, 14 = hard
    // 15 = wet
    pub m_visualTyreCompound: u8, // F1 visual (can be different from actual compound)
    // 16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
    // F1 Classic – same as above
    // F2 ‘19, 15 = wet, 19 – super soft, 20 = soft
    // 21 = medium , 22 = hard
    pub m_tyresAgeLaps: u8,     // Age in laps of the current set of tyres
    pub m_vehicleFiaFlags: i8,  // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow
    pub m_enginePowerICE: f32,  // Engine power output of ICE (W)
    pub m_enginePowerMGUK: f32, // Engine power output of MGU-K (W)
    pub m_ersStoreEnergy: f32,  // ERS energy store in Joules
    pub m_ersDeployMode: u8, // ERS deployment mode, 0 = none, 1 = medium, 2 = hotlap, 3 = overtake
    pub m_ersHarvestedThisLapMGUK: f32, // ERS energy harvested this lap by MGU-K
    pub m_ersHarvestedThisLapMGUH: f32, // ERS energy harvested this lap by MGU-H
    pub m_ersDeployedThisLap: f32, // ERS energy deployed this lap
    pub m_networkPaused: u8, // Whether the car is paused in a network game
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketCarStatusData {
    pub m_header: PacketHeader, // Header
    pub m_carStatusData: [CarStatusData; 22],
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct FinalClassificationData {
    pub m_position: u8,     // Finishing position
    pub m_numLaps: u8,      // Number of laps completed
    pub m_gridPosition: u8, // Grid position of the car
    pub m_points: u8,       // Number of points scored
    pub m_numPitStops: u8,  // Number of pit stops made
    pub m_resultStatus: u8, // Result status - 0 = invalid, 1 = inactive, 2 = active
    // 3 = finished, 4 = didnotfinish, 5 = disqualified
    // 6 = not classified, 7 = retired
    pub m_bestLapTimeInMS: u32, // Best lap time of the session in milliseconds
    pub m_totalRaceTime: f64,   // Total race time in seconds without penalties
    pub m_penaltiesTime: u8,    // Total penalties accumulated in seconds
    pub m_numPenalties: u8,     // Number of penalties applied to this driver
    pub m_numTyreStints: u8,    // Number of tyres stints up to maximum
    pub m_tyreStintsActual: [u8; 8], // Actual tyres used by this driver
    pub m_tyreStintsVisual: [u8; 8], // Visual tyres used by this driver
    pub m_tyreStintsEndLaps: [u8; 8], // The lap number stints end on
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketFinalClassificationData {
    pub m_header: PacketHeader, // Header
    pub m_numCars: u8,          // Number of cars in the final classification
    pub m_classificationData: [FinalClassificationData; 22],
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct LobbyInfoData {
    pub m_aiControlled: u8, // Whether the vehicle is AI (1) or Human (0) controlled
    pub m_teamId: u8,       // Team id - see appendix (255 if no team currently selected)
    pub m_nationality: u8,  // Nationality of the driver
    pub m_platform: u8,     // 1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown
    pub m_name: String,     // Name of participant in UTF-8 format – null terminated
    // Will be truncated with ... (U+2026) if too long
    pub m_carNumber: u8,   // Car number of the player
    pub m_readyStatus: u8, // 0 = not ready, 1 = ready, 2 = spectating
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketLobbyInfoData {
    pub m_header: PacketHeader, // Header
    pub m_numPlayers: u8,       // Number of players in the lobby data
    pub m_lobbyPlayers: [LobbyInfoData; 22],
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CarDamageData {
    pub m_tyresWear: [f32; 4],      // Tyre wear (percentage)
    pub m_tyresDamage: [u8; 4],     // Tyre damage (percentage)
    pub m_brakesDamage: [u8; 4],    // Brakes damage (percentage)
    pub m_frontLeftWingDamage: u8,  // Front left wing damage (percentage)
    pub m_frontRightWingDamage: u8, // Front right wing damage (percentage)
    pub m_rearWingDamage: u8,       // Rear wing damage (percentage)
    pub m_floorDamage: u8,          // Floor damage (percentage)
    pub m_diffuserDamage: u8,       // Diffuser damage (percentage)
    pub m_sidepodDamage: u8,        // Sidepod damage (percentage)
    pub m_drsFault: u8,             // Indicator for DRS fault, 0 = OK, 1 = fault
    pub m_ersFault: u8,             // Indicator for ERS fault, 0 = OK, 1 = fault
    pub m_gearBoxDamage: u8,        // Gear box damage (percentage)
    pub m_engineDamage: u8,         // Engine damage (percentage)
    pub m_engineMGUHWear: u8,       // Engine wear MGU-H (percentage)
    pub m_engineESWear: u8,         // Engine wear ES (percentage)
    pub m_engineCEWear: u8,         // Engine wear CE (percentage)
    pub m_engineICEWear: u8,        // Engine wear ICE (percentage)
    pub m_engineMGUKWear: u8,       // Engine wear MGU-K (percentage)
    pub m_engineTCWear: u8,         // Engine wear TC (percentage)
    pub m_engineBlown: u8,          // Engine blown, 0 = OK, 1 = fault
    pub m_engineSeized: u8,         // Engine seized, 0 = OK, 1 = fault
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketCarDamageData {
    pub m_header: PacketHeader, // Header
    pub m_carDamageData: [CarDamageData; 22],
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct LapHistoryData {
    pub m_lapTimeInMS: u32,       // Lap time in milliseconds
    pub m_sector1TimeInMS: u16,   // Sector 1 time in milliseconds
    pub m_sector1TimeMinutes: u8, // Sector 1 whole minute part
    pub m_sector2TimeInMS: u16,   // Sector 2 time in milliseconds
    pub m_sector2TimeMinutes: u8, // Sector 2 whole minute part
    pub m_sector3TimeInMS: u16,   // Sector 3 time in milliseconds
    pub m_sector3TimeMinutes: u8, // Sector 3 whole minute part
    pub m_lapValidBitFlags: u8,   // 0x01 bit set - lap valid, 0x02 bit set - sector 1 valid
                                  // 0x04 bit set - sector 2 valid, 0x08 bit set - sector 3 valid
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct TyreStintHistoryData {
    pub m_endLap: u8,             // Lap the tyre usage ends on (255 of current tyre)
    pub m_tyreActualCompound: u8, // Actual tyres used by this driver
    pub m_tyreVisualCompound: u8, // Visual tyres used by this driver
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketSessionHistoryData {
    pub m_header: PacketHeader,                // Header
    pub m_carIdx: u8,                          // Index of the car this lap data relates to
    pub m_numLaps: u8,       // Num laps in the data (including current partial lap)
    pub m_numTyreStints: u8, // Number of tyre stints in the data
    pub m_bestLapTimeLapNum: u8, // Lap the best lap time was achieved on
    pub m_bestSector1LapNum: u8, // Lap the best Sector 1 time was achieved on
    pub m_bestSector2LapNum: u8, // Lap the best Sector 2 time was achieved on
    pub m_bestSector3LapNum: u8, // Lap the best Sector 3 time was achieved on
    pub m_lapHistoryData: Vec<LapHistoryData>, // 100 laps of data max
    pub m_tyreStintsHistoryData: [TyreStintHistoryData; 8],
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct TyreSetData {
    pub m_actualTyreCompound: u8, // Actual tyre compound used
    pub m_visualTyreCompound: u8, // Visual tyre compound used
    pub m_wear: u8,               // Tyre wear (percentage)
    pub m_available: u8,          // Whether this set is currently available
    pub m_recommendedSession: u8, // Recommended session for tyre set
    pub m_lifeSpan: u8,           // Laps left in this tyre set
    pub m_usableLife: u8,         // Max number of laps recommended for this compound
    pub m_lapDeltaTime: i16,      // Lap delta time in milliseconds compared to fitted set
    pub m_fitted: u8,             // Whether the set is fitted or not
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketTyreSetsData {
    pub m_header: PacketHeader,           // Header
    pub m_carIdx: u8,                     // Index of the car this data relates to
    pub m_tyreSetData: [TyreSetData; 20], // 13 (dry) + 7 (wet)
    pub m_fittedIdx: u8,                  // Index into array of fitted tyre
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PacketMotionExData {
    pub m_header: PacketHeader, // Header

    // Extra player car ONLY data
    pub m_suspensionPosition: [f32; 4], // Note: All wheel arrays have the following order:
    pub m_suspensionVelocity: [f32; 4], // RL, RR, FL, FR
    pub m_suspensionAcceleration: [f32; 4], // RL, RR, FL, FR
    pub m_wheelSpeed: [f32; 4],         // Speed of each wheel
    pub m_wheelSlipRatio: [f32; 4],     // Slip ratio for each wheel
    pub m_wheelSlipAngle: [f32; 4],     // Slip angles for each wheel
    pub m_wheelLatForce: [f32; 4],      // Lateral forces for each wheel
    pub m_wheelLongForce: [f32; 4],     // Longitudinal forces for each wheel
    pub m_heightOfCOGAboveGround: f32,  // Height of centre of gravity above ground
    pub m_localVelocityX: f32,          // Velocity in local space – metres/s
    pub m_localVelocityY: f32,          // Velocity in local space
    pub m_localVelocityZ: f32,          // Velocity in local space
    pub m_angularVelocityX: f32,        // Angular velocity x-component – radians/s
    pub m_angularVelocityY: f32,        // Angular velocity y-component
    pub m_angularVelocityZ: f32,        // Angular velocity z-component
    pub m_angularAccelerationX: f32,    // Angular acceleration x-component – radians/s/s
    pub m_angularAccelerationY: f32,    // Angular acceleration y-component
    pub m_angularAccelerationZ: f32,    // Angular acceleration z-component
    pub m_frontWheelsAngle: f32,        // Current front wheels angle in radians
    pub m_wheelVertForce: [f32; 4],     // Vertical forces for each wheel
}