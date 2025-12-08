pub(super) static PROJECTS: &str = r#"# Projects
   The easiest way to see what I've been up to lately is to check out my GitHub profile: *https://github.com/Dark-Alex-17*

## Software Sideprojects
* **Managarr**: A Rust-based TUI for managing many different Servarr instances (*https://github.com/Dark-Alex-17/managarr*)
   * You can test out *Managarr* by visiting the demo site: (*https://managarr-demo.alexjclarke.com*)
* **G-Man**: Universal command line credential management and injection tool (*https://github.com/Dark-Alex-17/gman*)
* **Loki**: An All-in-one, batteries included LLM CLI tool (*https://github.com/Dark-Alex-17/loki)
* **devtools**: An AIO CLI for your command line tasks: cloud management (AWS/GCP), databases, AI tools, plotting, system maintenance, and more (*https://github.com/Dark-Alex-17/dtools)
* **ClaSH**: This shell and website (*https://github.com/Dark-Alex-17/ClaSH*)
* **DynamoDB and DAX Benchmarker**: A simple benchmarking utility built in both Rust and Go to benchmark performance differences between a DynamoDB table that doesn't use DAX, and one that does. An Elastic stack is used to receive the benchmarking metrics in real time so that users can also visualize the benchmarking results in Kibana using the pre-built dashboard. (*https://github.com/Dark-Alex-17/dynamodb-dax-benchmarker*)
* **cp_sat**: Rust FFI bindings for the Google OR-Tools CP-SAT Solver to allow using the OR-Tools CP-SAT solver in Rust. (*https://github.com/Dark-Alex-17/cp_sat*)
* **AWS Cloud Gaming**: A Bash TUI built with whiptail to provision an EC2 instance in AWS, configure it with Steam, set up a low-latency RDP session using AWS' own NICE DCV to play games using AWS EC2 instances. (*https://github.com/Dark-Alex-17/aws-cloud-gaming*)
* **Alerting-API**: A simple reactive API that utilizes the Quartz library to provide a dynamic, reactive alerting system (minus the notification component), built with Kotlin. (*https://github.com/Dark-Alex-17/alerting-api*)

## Homelab
### Gist
I have roughly 50 different servers that I self-host and experiment with. I've exposed a few publicly for family and friends to use. You can see a sliver of what I host and tinker with regularly here:

*https://dashboard.alexjclarke.com*

And to see the status of my publicly-exposed services, you can check my status page:

*https://status.alexjclarke.com*

I've created a CLI that essentially wraps the state management and deployment of every server in my network using Ansible and Terraform for automated deployments and IaC. The basic usage is simply: Ansible (pre-provisioning) + Terraform (service deployments) + Ansible (post-provisioning or Ansible-exclusive deployments). My repo includes the usage of Ansible Vault for secrets management (so I can keep AWS costs down and rely on only one secrets manager for both Ansible and Terraform), which is the reason I haven't open-sourced the repo.

Alongside each server, I also deploy automated backups that follow the 3-2-1 rule and a suite of monitoring tools to notify me if ever anything goes down.

### Current project: Meshtastic
Currently I'm working on setting up a handful of [Meshtastic](https://meshtastic.org) nodes around my state so that I can forward MQTT messages through it. The purpose of this is so that I can always monitor my network and control my network and servers, even if the internet itself is out and my network is inacessible from the outside world.

If you have any questions about my homelab, don't hesitate to reach out! I'm always happy to talk about it!

## Mathematics Sideprojects
* **Real Analysis - Theorems and Definitions**: A large LaTeX book that's designed to be a quick-reference for many theorems, definitions, lemmas, corrollaries, etc. that are useful when proving conjectures or when studying properties of real numbers. (*https://github.com/Dark-Alex-17/real-analysis-theorems-and-definitions*)
* **Abstract Algebra - Theorems and Definitions**: Like the previous project, this one is the same idea, except it's a collection of theorems, definitions, etc. in Abstract Algebra for quick-reference. (*https://github.com/Dark-Alex-17/abstract-algebra-theorems-and-definitions*)
* **Linear Algebra - Theorems and Definitions**: Like the previous two projects, this one is the same idea, except it's a collection of theorems, definitions, etc. in Linear Algebra for quick-reference. It's proven to be incredibly useful when designing models, or when optimizing database queries. (*https://github.com/Dark-Alex-17/linear-algebra-theorems-and-definitions*)

Outside of these sideprojects, I do recreational math when time permits and often find myself incredibly deep down those wonderful rabbit holes.

## Piano
* In my free time, I also love to play the piano. I've posted some performances to my TikTok for anyone curious enough to look: (*https://www.tiktok.com/@alextusa0*)
"#;
