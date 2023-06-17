const completion: Fig.Spec = {
  name: "acai",
  description: "Fast package manager for macOS apps",
  subcommands: [
    {
      name: "install",
      description: "Installs packages",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
      args: {
        name: "PACKAGE",
      },
    },
    {
      name: "search",
      description: "Searches seeds",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
      args: {
        name: "QUERY",
        isOptional: true,
      },
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      subcommands: [
        {
          name: "install",
          description: "Installs packages",
        },
        {
          name: "search",
          description: "Searches seeds",
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
        },
      ],
    },
  ],
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help",
    },
  ],
};

export default completion;
