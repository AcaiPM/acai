//
//  Software.swift
//  acai
//
//  Created by Staturnz on 9/28/23.
//

#if os(macOS) || os(iOS) || os(tvOS) || os(watchOS)
import Foundation
#endif

public class software {
    public enum operatingSystemType {
        case macOS
        case iOS
        case watchOS
        case tvOS
        case Linux
        case FreeBSD
        case PS4
        case Windows
        case OpenBSD
        case Cygwin
        case WASI
        case Haiku
        case Android
        case unknown
    }
    
    public static func operatingSystem() -> operatingSystemType {
        #if os(macOS)
            return .macOS
        #elseif os(iOS)
            return .iOS
        #elseif os(watchOS)
            return .watchOS
        #elseif os(tvOS)
            return .tvOS
        #elseif os(Linux)
            return .Linux
        #elseif os(FreeBSD)
            return .FreeBSD
        #elseif os(PS4)
            return .PS4
        #elseif os(Windows)
            return .Windows
        #elseif os(OpenBSD)
            return .OpenBSD
        #elseif os(Cygwin)
            return .Cygwin
        #elseif os(WASI)
            return .WASI
        #elseif os(Haiku)
            return .Haiku
        #elseif os(Android)
            return .Android
        #else
            return .unknown
        #endif
    }
    
    public static func operatingSystemVersion() -> String {
        #if os(macOS) || os(iOS) || os(tvOS) || os(watchOS)
            if #available(macOS 10.10, iOS 8.0, watchOS 3.0, tvOS 8.0, *) {
                return Foundation.ProcessInfo().operatingSystemVersionString
            } else {
                // TODO: Other detection methods
            }
        #elseif os(Linux) || os(Haiku) || os(Android) || os(FreeBSD) || os(OpenBSD) || os(PS4)
            var systemInfo = utsname()
            uname(&systemInfo)
        
            let mirror = Mirror(reflecting: systemInfo.release)
            let releaseVersion = mirror.children.reduce("") { ident, element in
                guard let value = element.value as? Int8, value != 0 else { return ident }
                return ident + String(UnicodeScalar(UInt8(value)))
            }
        #elseif os(Cygwin) || os(Windows)
            // TODO: Add Windows version detection
            return "Windows"
        #else
            return "Unknown system version"
        #endif
    }
}
