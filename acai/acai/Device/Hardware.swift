//
//  Hardware.swift
//  acai
//
//  Created by Staturnz on 9/28/23.
//


public class hardware {
    public enum architectureType {
        case x86_64
        case x86
        case PowerPC64
        case Arm
        case Arm64
        case Arm64_32
        case unknown
    }
    
    public enum endianType {
        case big
        case little
        case mixed
        case unknown
    }
    
    public static func architecture() -> architectureType {
        #if arch(x86_64)
            return .x86_64
        #elseif arch(i386)
            return .x86
        #elseif arch(powerpc64le)
            return .PowerPC64
        #elseif arch(arm)
            return .Arm
        #elseif arch(arm64)
            return .Arm64
        #elseif arch(arm64_32)
            return .Arm64_32
        #else
            return .unknown
        #endif
    }
    
    public static func endian() -> endianType {
        #if _endian(big)
            return .big
        #elseif _endian(little)
            return .little
        #elseif _endian(big) && _endian(little)
            return .mixed
        #else
            return .unknown
        #endif
    }
}
