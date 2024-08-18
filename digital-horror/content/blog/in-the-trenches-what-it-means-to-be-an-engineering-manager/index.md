+++
title = "In the Trenches: What it Means to be an Engineering Manager"
description = "Personal reflections on my past role as an Engineering Manager."
date = 2024-08-17
[taxonomies]
tags = ["leadership", "management"]
+++

This is my first attempt at a non-technical piece. It's mostly a collection of thoughts which hopefully guides readers looking to get become engineering managers, leaders or simply better engineers. Another motivating factor is help improve the lack of content around the subject, and perhaps the negative perception that Eng. Managers, or middle-management in general, gets within our industry.

## Context

I have recently resigned as the Head of Engineering at a growing cybersecurity startup after a three year stint. Back when I joined, we had no platform, product, infrastructure, or processes. Today, with a very lean team, we've built out a scalable and reliable infrastructure, two enterprise products, one of which offers soft realtime event streaming of thousands of embedded devices and processing of tens of thousands of events per second with minimal compute. If I had to summarise the stack, it would be Rust, Redpanda (Kafka) and Timescale (Postgres).

Nearly everyone is, to a certain degree, an Engineer. Engineers are harbingers of change. Individuals who achieve what they want, with what they have, to problems that matter. Software engineers naturally fall within this category and in my experience, contrary to many, working with capable engineers has been the most pleasant, mentally engaging work I have done in my career. It's tough, but the results that come out of a good team is inspiring.

## People

Often times I find people separating [information] technology from humanities. Arguing that technology does not concern itself with societal problems and therefore since not be exposed to it (remember the move from `master` to `main` in git, or `master`/`slave` rename in [redis](https://github.com/redis/redis/issues/5335)). If we are to agree on our definition of an Engineer, the counterfactual is simple. We solve problems that matter, and problems that matter are ones that positively affect the people in the world and around us.

Similarly, engineers and managers also detach themselves from dealing with the personal; often redirecting attention to sterile KPIs, metrics, OKRs as a proxy for good, or bad, work. The guideline to the aspiring manager varies depending on the size and experience of the team you are managing.

Understand that your role as the leader of your team is that of a support person. Supporting your engineers to perform their best in a way that enables the business to achieve its mission. Turns out that most of the time, this requires little to no technical involvement, and much more personal involvement. You're not expected to be a therapist, or a close friend, but you must be able to listen carefully. Understand the various personalities in your team (i.e., leaders, warriors and talent) and gradually align their incentives, with the organisations expectation.

<details>
  <summary>The Hyper-technical Engineer Case Study</summary>

>  The hyper-technical engineers (≈10x engineer). Often focusing solely on technical problems with little concern to the organisation's problems writ large. They are exceptionally capable and often times require to play give-and-take in order to progress. Their primary incentive is technical excellence, and as such will tend to over-work themselves in order to achieve a technical goal (e.g., large refactor). The first two mistakes would be to (a) allow such engineers to take on most or all of the work on a project and (b) not challenge the work that they want to focus on rigorously.
>
>  The first mistake is tackled by pushing ownership and autonomy to other engineers on the project that this engineer would want to tackle themselves (work stealing, if you will). At face value this will seem like slowing progress, but in fact it increases quality and resilience of both the team and the product; particularly from the inevitable burnout that would otherwise occur. Additionally it cultivates teamwork, and helps uplift other engineers who may feel like they're not up to the task.
>
>  The second mistake is overcome by guiding the engineer back and presenting to them the entire picture of the organisation and where their work falls in it. Once you have a macro view, you can present the case to them in a way that allows us to get to the ~right~ desired conclusion together. For example, should we implement a new event streaming pipeline that uses zero-cost abstractions[^1] between a source (e.g., redpanda) and sink (e.g., timescale), optimising the least-bottlenecked part of our system? Or should we implement the simple data exporter that allows our very first partner to integrate with our platform?
>
>  The first option is fun and exciting. We get to play with the Rust type system and watch as the streaming process shatters past benchmarks. It does not however, further progress the organisation in achieving its mission. The second option is more trivial, and mostly a matter of labour which significantly contributes to the organisation's mission and success. We may get to the natural conclusion that the second option is more important and stop there. The problem with this is that you'll gradually lose the motivation from this engineer which leads them to not recommend highly technical and creative solutions while being more likely to leave the team.
>
>  We have taken their desired outcome, and therefore should try give them what they really wanted. The data exporter may be more trivial, but you may also draw comparisons to the first option. You need to move events around from your system to another system without requiring mutating data in transit. This would likely benefit from the similar zero-cost abstractions mentioned in the first option. Giving this to the engineer sparks their curiosity again and results in even further improvements. The result is a data streaming exporter that has minimal latency and overhead which in the context of a security detection and response platform, is considered a key feature. Partners feel excited to work with such performant and well engineered solutions, the business is happy its strengthening the relationship with its partner and most importantly, the engineer contributed to a piece of work that is not only tecnically engaging, but truly valuable.
>
>  This is but one example, I mention the hyper-technical engineer as they are folks that often lead to managerial traps which take a while to manifest and are extremely difficult to correct. The same principles of understanding your engineers, what makes them tick, what their work output looks like (i.e., cyclical or linear) and how they are doing should be applied to all the engineers in the team.

</details>

## Team

Defend ideas passionately. Occassionally strongly disagree with people they are working with. Not an easy balance, but important.

## Technology

Mauris vehicula enim lorem, condimentum pulvinar orci scelerisque a. Curabitur molestie feugiat tristique. Phasellus sed dolor id libero mollis congue nec a ligula. Morbi sit amet ullamcorper orci. Nulla tempor leo at felis tincidunt pellentesque. Proin sodales dignissim turpis vel porttitor. Etiam vel ipsum eget nunc sodales aliquet. Duis molestie vitae justo at mattis. Vivamus sagittis, eros eget maximus laoreet, tortor nulla ullamcorper velit, sed consectetur metus neque a arcu. Duis at nisi massa. Nulla tempus justo nec dui mollis, non gravida velit porta. Etiam euismod lorem et dui venenatis, quis sodales arcu aliquet. Cras hendrerit vulputate magna, ut blandit est vulputate in. Duis diam eros, fermentum placerat varius id, tincidunt vel est. Sed rutrum auctor nibh a aliquam.

## Leaders Eat Last

Support. Enabler.

Sed tincidunt felis justo, eget blandit quam malesuada et. Sed bibendum ultrices lacus id bibendum. Etiam et eros lorem. Cras odio sapien, tristique sed elementum id, elementum quis magna. Donec viverra nisi faucibus turpis posuere, ac dapibus odio accumsan. Vestibulum lacinia eleifend nulla. Nullam eget suscipit lacus. Pellentesque vitae risus at nisi commodo placerat. Suspendisse sit amet imperdiet enim. Quisque id dui venenatis, tempus justo vel, lacinia metus.

## Importance of Business Acumen

Vestibulum interdum nunc at eros semper convallis. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Mauris venenatis, ligula at pulvinar luctus, lectus tortor mattis est, et condimentum turpis nisi interdum leo. Nunc ut fringilla velit. Phasellus eleifend dolor ut libero cursus, eget luctus nulla iaculis. Duis dapibus tristique convallis. Fusce commodo at turpis et mattis. Nunc molestie ipsum a posuere dapibus. In posuere felis sed ipsum molestie congue. Morbi vel ipsum imperdiet, dapibus dui nec, tempor metus. Etiam vestibulum justo a pulvinar faucibus. In hac habitasse platea dictumst.

## On Adapting

Adapting to your business, team and market.

Suspendisse sit amet enim id sapien ornare dapibus sit amet id velit. Nam venenatis eleifend justo eu rutrum. Pellentesque scelerisque lacinia sem vel varius. Sed vel eleifend odio. In congue ut ligula vitae fringilla. Duis tristique enim viverra felis tincidunt, eu pharetra nisi vehicula. Sed augue nulla, pretium in justo et, tincidunt sodales enim. Pellentesque eu velit et est malesuada porta a sed dui. Aenean sit amet tortor iaculis, tempor eros et, aliquet nulla. Fusce turpis nibh, imperdiet non ultrices blandit, tincidunt id velit. Nulla pellentesque nisi nec odio ultricies, ut interdum dui tristique. Proin lacinia sapien sed commodo convallis.

# Conclusion

Suspendisse sit amet enim id sapien ornare dapibus sit amet id velit. Nam venenatis eleifend justo eu rutrum. Pellentesque scelerisque lacinia sem vel varius. Sed vel eleifend odio. In congue ut ligula vitae fringilla. Duis tristique enim viverra felis tincidunt, eu pharetra nisi vehicula. Sed augue nulla, pretium in justo et, tincidunt sodales enim. Pellentesque eu velit et est malesuada porta a sed dui. Aenean sit amet tortor iaculis, tempor eros et, aliquet nulla. Fusce turpis nibh, imperdiet non ultrices blandit, tincidunt id velit. Nulla pellentesque nisi nec odio ultricies, ut interdum dui tristique. Proin lacinia sapien sed commodo convallis.

[^1]: https://www.reddit.com/r/rust/comments/bo13qq/what_specifically_are_all_the_zerocost/
