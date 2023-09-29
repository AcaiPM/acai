//
//  Capability.swift
//  acai
//
//  Created by Staturnz on 9/28/23.
//

#if os(Windows) || os(Cygwin)
public typealias BYTE = UInt8
public typealias WORD = Int16
public typealias DWORD = Int
public typealias QWORD = Int64
public typealias LONG = Int
public typealias LONGLONG = Int64
public typealias ULONGLONG = UInt64
#endif

public class capability {
    public enum floatingPoints {
        case Float16
        case Float80
        case Float128
    }
}

