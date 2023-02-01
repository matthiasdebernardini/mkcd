# mkcd
A cli that does not work;

It does not work because of the way that unix handles processes. When you invoke a command in the CLI, the process that handled that prompt (be it cd or echo)



┌─────────┐   ┌──────────┐         ┌─────────┐
│         │   │          │         │         │
│  shell  ├──►│  fork()  ├────┬───►│  shell  │◄──────────────────────── mkcd canot access this
│         │   │          │    │    │         │                                    │
└────▲────┘   └──────────┘    │    └─────────┘                                    │
     │                        │                                                   │
     │                        │                                                   │
     │                        │    ┌────────────────┐     ┌─────────────┐     ┌────────┐
     │                        │    │                │     │             │     │        │
     │                        └────►  copy of shell ├────►│  exec(mkcd) ├─────►  mkcd  │
     │                             │                │     │             │     │        │
     │                             └────────────────┘     └─────────────┘     └─────┬──┘
     │                                                                              │
     │                                                                              │
     └──────────────────────────────────────────────────────────────────────────────┘
