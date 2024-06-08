+++
+++

# Juxhin Dyrmishi Brigjaj &mdash; Curriculum Vitae

## Professional Summary

Highly skilled and innovative technical leader with 10+ years of experience in software engineering, cybersecurity, and engineering management. Proven track record in building and leading engineering teams, developing robust software solutions, and driving technological advancements. Expert in Rust, cloud infrastructure, and cybersecurity. Passionate about building scalable platforms and enabling innovation through strategic leadership.

## Professional Experience

### Head of Engineering, Senior Software Engineer
[**Exein SpA**](https://exein.io/)
*Nov 2021 – Current*

- Led and managed the Engineering division, driving the development and deployment of cutting-edge cybersecurity solutions.
- Designed and developed [Exein Runtime](https://www.exein.io/solution/exein-runtime/), an event-driven, soft realtime platform enabling analysts and operators to manage the security posture of thousands of devices in the wild. Powered by Rust, TimescaleDB & Redpanda.
    - Platform has processed over 40 billion security events across a vast fleet of devices, handling tens of millions of events daily.
    - Ability to support over 73k events per second on a single 2vCPU & 2048MB memory ECS container utilizing Rust's zero-copy abstractions and gRPC.
- Designed and developed the rewrite of [Exein Analyzer](https://www.exein.io/solution/iot-security-analysis/), reducing total scan time by 70%+ by employing a [supervisor model](https://www.erlang.org/doc/apps/stdlib/supervisor.html) for the distribution of scan jobs — enabling scalability and resilience.
- Designed and implemented AWS cloud infrastructure, ensuring high availability, scalability and establishing SLAs.
- Collaborated with product and executive teams to align engineering efforts with business goals.
- Mentored and developed senior engineers, fostering a culture of continuous learning and innovation.

### Advisor, Co-founder & Chief Engineer
[**PhishDeck**](https://phishdeck.com/)
*September 2019 – Current*

- Led the engineering of PhishDeck, a cybersecurity company specializing in phishing simulation platforms.
    - Designed, and developed [Phinn](https://www.phishdeck.com/blog/phinn-on-engineering-a-real-time-phishing-simulation-proxy/), the first-of-its-kind real-time phishing proxy that enables the most realistic phishing attacks in the market to date.
    - Designed, developed and deployed the platform, powered by Python (Flask & uWSGI), Postgres, and RabbitMQ.
    - Infrastructure as Code (IaC) using Terraform and Ansible on DigitalOcean.
- Led the company's business development opportunities.
    - Spearheaded the acquisition of large clients including [Wildlife Studios](https://wildlifestudios.com/) and [GiG](https://phishdeck.com/case-studies/gig-gaming).
    - Led the acquisition of PhishDeck by ThreatAssure from the start till closing the acquisition.
- Post-acquisition, continued to provide strategic guidance for the technical and business growth of the organization.
    - Guided the technical and product roadmap.
    - Developed and monitored organizational OKRs/KPIs around sales and business development alongside new leadership.

### Security Architect, Application Security Engineer
[**Gaming Innovation Group**](https://www.gig.com/)
*October 2018 – Nov 2021*

- Established and enforced organization-wide security frameworks, focusing on secure coding practices and architectural design.
    - Achieved OWASP SAMM Level 3 Maturity across Implementation, Verification and Operation verticals.
    - Deployed Checkmarx, Nessus and Netsparker across core services and assets within CI/CD infrastructure.
- Enhanced incident response capabilities including advanced threat modeling and threat hunting exercises.
    - Organized first threat modeling workshop in [2021](https://www.gig.com/sustainability/it-and-cybersecurity/) with all Tech/Engineering leadership.
    - Architected and deployed opaque log monitoring strategy as a means of consolidating all platform traffic with 40 B2B customers.
    - Key stakeholder in the acquisition, design and deployment of [Splunk](https://www.splunk.com/) across all services and corporate assets.
- Conducted technical security risk assessments for third-party integrations, ensuring robust security measures.
    - Discovered and disclosed multiple critical severity vulnerabilities (Out-of-band XXE, SQLi, Object Deserialization, Unauthenticated RCE).
    - Led fraud investigation, detecting and mitigating a loss of €2 million related affecting multiple B2B clients.
- Contributed to the architectural design of the new core system, integrating comprehensive security protocols.
    - Designed and deployed centralized identity provider (IdP) across 6 B2B products, enabling single identity across entire product portfolio.
    - Along with Enterprise Architects, designed AuthN/AuthZ for all users and services in Enterprise OpenShift platform.
- Developed and implemented a comprehensive Vulnerability Management process, managing security vulnerabilities across all business verticals and security areas.
    - Established internal SLAs for vulnerability remediation and deployment across all engineering and operational leaders.
    - Established the company's [Security Vulnerability Disclosure](https://www.gig.com/security-vulnerability-disclosure-policy/) policy as a precursor to developing a bug bounty program.
- Led the research, design, and implementation of critical Product Security features, enhancing market competitiveness.
    - Designed and deployed TOTP & HOTP 2FA for 30,000+ end-users across 40 brands.
    - Designed Geolocation-based MFA to safeguard end-users against account takeover attempts.

### Backend Engineer, Technical Writer
[**Acunetix**](https://www.acunetix.com/)
*2015 – 2018*

- Led the design and development of critical features for the main product line: Development and deployment of [Acunetix AcuMonitor](https://www.acunetix.com/vulnerability-scanner/acumonitor-technology/).
    - [Integrations](https://www.acunetix.com/support/docs/wvs/integrating-acunetix-with-azure-devops-server-tfs/) with GitHub, JIRA, and Microsoft TFS.
    - Developed Python backend services and PHP components, leveraging Docker, Docker Compose, and Vagrant.
- Built a custom DNS resolver capable of handling 40k DNS queries per second[[1](https://github.com/JuxhinDB/OOB-Server)][[2](https://blog.digital-horror.com/blog/synner-a-tcp-syn-client-written-in-rust/)].
- Implemented test-driven development practices, focusing on optimizing small, well-defined systems.
    - Developed the company's first comprehensive CI test suite, utilizing Docker during its nascent stage to run quality and regression tests against each build of our security scanner.
- Performed security analysis for high-profile clients to identify and mitigate potential vulnerabilities.
    - Assisted agencies and organizations including NASA, US Airforce, Canadian Tire and American Express to consolidate large quantity of application security vulnerabilities into key actionable tasks for their teams.
    - Held multiple conference calls with high-value clients, providing training and consultation to 8+ engineers at a time.

## Education

### Master's degree, Software Engineering
**University of Oxford**
*2021 – 2024*

- On track to achieve a First-class Honours.
- Research: Sound orbital computing platforms in LEO using Rust; Presentation-based user authentication for the next generation digital identity.

### Bachelor's degree, Computer Information Systems
**University of London**
*2015 – 2020*

- Achieved a First-class Honours with multiple awards throughout the degree.
- Research: Deferred Rendering 3D Engine pipelines; MitM Network Phishing Proxies that bypass 2FA.

## Skills

These are non-exhaustive but aim to provide an idea into what technology I cover.

##### Programming Languages
Rust, Python, C, Go, Erlang, Haskell, Java, Ada, CSP*, TLA+*.

##### Cloud Providers
AWS, GCP and DigitalOcean

##### Cybersecurity
AppSec, Penetration Testing & Vulnerability Assessment, Secure Coding Practices, Out-of-band exploits.

##### Leadership
Team Management, Strategic Planning, Mentorship.

##### Tools & Technologies
Docker, Kubernetes, Terraform, AWS CDK, Ansible, eBPF.

##### Embedded
ESP32.

##### Storage
Postgres, TimescaleDB, MongoDB, MariaDB, Cassandra, InfluxDB, SQLite.

##### Streaming
Redpanda, Kafka and RabbitMQ.

##### Auxiliary Tools
[neo]vim, lldb, git, tmux, btop.

##### Miscellaneous

3D Printing, CAD, PCB design.
