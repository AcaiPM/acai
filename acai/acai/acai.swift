//
//  main.swift
//  acai
//
//  Created by Staturnz on 9/28/23.
//

#if os(macOS) || os(iOS) || os(tvOS) || os(watchOS)
import Foundation
#endif

public var stderr = FileHandle.standardError
public var stdout = FileHandle.standardOutput

extension FileHandle: TextOutputStream {
  public func write(_ string: String) {
    guard let data = string.data(using: .utf8) else {return}
    self.write(data)
  }
}


private func printUsage(_ path: String) {
    let usageMessage = """
    Usage: \(path) <option> <sub-option>
        
        --install, -i       install a package
        --search, -s        search for a package
    """
    
   print(usageMessage, to: &stderr)
}


@main struct entry {
    static func main() -> Void {
        let args = CommandLine.arguments
        exit(main(args.count, args))
    }
    
    public static func main(_ argc: Int,_ argv: [String]) -> Int32 {
        if (argc <= 1) {
            printUsage(argv[0])
        } else if (argc > 1 && argc <= 3) {
            switch (argv[1].lowercased()) {
            case "--install", "-i":
                break
            case "--search", "-s":
                break
            default: break
            }
        }
        return 0
    }
}

