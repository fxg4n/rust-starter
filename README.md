flowchart TD
  %% Interface Layer
  subgraph "Client Interfaces"
    HTTP[HTTP API]
    GraphQL[GraphQL API]
    WS[WebSocket]
  end

  subgraph "Interface Adapters (interfaces)"
    InterfaceLayer[Converts external requests to internal commands/queries]
  end

  %% Application Layer
  subgraph "Application Services (application)"
    AppLayer[Use Cases<br/>DTOs<br/>Application Services]
  end

  %% Domain Layer
  subgraph "Domain Logic (domain)"
    Domain[Entities<br/>Value Objects<br/>Business Rules]
  end

  %% Ports and Adapters
  subgraph "Ports (Traits)"
    Ports[Interfaces for DB, services, etc.]
  end

  subgraph "Infrastructure (infrastructure)"
    Infra[Adapters for Repos<br/>SMTP<br/>Third-party APIs]
  end

  subgraph "External Systems"
    DB[(PostgreSQL)]
    SMTP[(SMTP Service)]
    API[(Third-party APIs)]
  end

  %% Core & Shared
  subgraph "Core Utilities (core)"
    CoreErrors[Error Handling]
    CoreSecurity[Security/Auth]
    CoreValidation[Validation]
  end

  subgraph "Drivers"
    DBAdapter[Database Adapter]
    Logging[Logging / Tracing]
    SMTPAdapter[SMTP Client]
  end

  subgraph "Shared Utils (shared)"
    Async[Async Tools]
    StringUtils[String / Time Utilities]
  end

  subgraph "Application Entry (app)"
    AppMain[App Entry<br/>Bootstrapping<br/>DI<br/>Server Config]
  end

  %% Connections
  HTTP --> InterfaceLayer
  GraphQL --> InterfaceLayer
  WS --> InterfaceLayer
  InterfaceLayer --> AppLayer
  AppLayer --> Domain
  Domain --> Ports
  Ports --> Infra
  Infra --> DB
  Infra --> SMTP
  Infra --> API

  AppMain --> InterfaceLayer
  AppMain --> AppLayer
  AppMain --> Infra
  AppMain --> CoreErrors
  AppMain --> CoreSecurity
  AppMain --> CoreValidation

  Infra --> DBAdapter
  Infra --> Logging
  Infra --> SMTPAdapter

  CoreErrors --> AppLayer
  CoreSecurity --> AppLayer
  CoreValidation --> AppLayer

  Async --> AppLayer
  StringUtils --> AppLayer
