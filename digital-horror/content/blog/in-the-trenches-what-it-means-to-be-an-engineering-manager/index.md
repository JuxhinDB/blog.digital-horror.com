+++
title = "In the Trenches: What it Means to be an Engineering Manager"
description = "Personal reflections on my past role as an Engineering Manager."
date = 2024-08-17
[taxonomies]
tags = ["leadership", "management"]
+++

This post is a collection of personal reflections from my time working as an EM and similar roles. It’s not meant to be a comprehensive guide but rather a snapshot of the lessons I’ve learned and the challenges I’ve faced. For those looking to transition into leadership, or even for engineers wanting to understand management better, I hope this offers some insight into the realities of the role.

Throughout my career, I’ve worked with some of the most capable engineers, and while it’s been challenging, it’s also been the most mentally engaging work I’ve done. A well-functioning team can achieve incredible results, and managing one is tough but inspiring work.

## People

Engineers and EMs often detach the technical from the personal, focusing on KPIs, metrics, and OKRs as proxies for success. While these are useful, it’s easy to overlook the human element that drives them. As an EM, your role isn’t just about managing the technical—it’s about understanding the people behind the work.

Technology impacts people, and as leaders, we can’t ignore that. Whether it's debates like the shift from `master` to `main` in git or deeper discussions around inclusivity in language (`master`/`slave` rename in redis[^1]), these things can shape team culture. I’ve seen firsthand how thoughtfully handling these conversations can strengthen cohesion and foster a culture of respect and alignment with the broader mission.

Your role is to enable your engineers to do their best work, often through personal involvement rather than technical input. You're not expected to be a life coach or a close friend, but you do need to listen carefully. Understand your team’s unique and diverse personalities, and gradually align their goals with the organization’s mission.

<details>
  <summary>Talented Specialist Engineer Case Study</summary>

> The talented specialist engineer often focuses exclusively on technical problems, sometimes overlooking the larger organizational goals. They are highly capable but require a careful balance of give-and-take to keep both them and the team moving forward. The first mistake is allowing them to take on all the work themselves. The second is failing to challenge their focus when it doesn't align with the broader mission.
>
> Tackling the first mistake involves distributing ownership and autonomy across the team, even if it feels like progress might slow down. While this can seem counterintuitive, it prevents burnout and builds resilience within both the team and the product. Encouraging collaboration also boosts confidence and skills in other engineers, who may otherwise feel overshadowed.
>
> The second mistake is corrected by presenting the bigger picture. As an example, when faced with two options &mdash; building an advanced event streaming pipeline leveraging zero-cost abstractions[^2] or creating a simple data exporter for a key partner &mdash; the technical specialist might lean toward the more exciting technical challenge. However, while the first option is fun, it may not move the business forward. The second, though less glamorous, directly impacts the company’s goals.
>
> That said, it’s important not to dismiss their technical curiosity outright. By framing the simpler task in a way that engages their problem-solving skills (e.g., focusing on performance or architectural improvements), you can keep them motivated while aligning their work with the business’s needs. In the end, they produce a technically elegant solution that both satisfies their creative drive and delivers real value to the company.

</details>

### Team

Building a strong team isn’t just about hiring for skill; it’s about recognizing that humans are primarily social thinkers, not first principle thinkers. We’re shaped by the people around us and the culture we operate in. This truth should influence how you hire. When we talk about "cultural fit," it’s not about hiring people who think exactly like you &mdah; that likely leads to stagnation. Instead, it’s about hiring people who share traits that matter to your team: honesty, hard work, respect, and healthy competition.

I’ve found that the biggest pitfall for engineering managers is hiring for comfort, bringing in folks who mirror their own thinking or who won’t challenge them. This is a dangerous trap. You should be hiring engineers who are better than you, especially in areas where you’re weak. That sounds obvious, but the reality is it can trigger a sense of insecurity or imposter syndrome.

Overcoming that is a personal journey. For me, it’s about understanding that I bring a lot to the table both managerially and technically. I remind myself that my role isn’t to be the most skilled person in the room but to create an environment where the most skilled people can thrive. That mindset shift helped me not only embrace hiring top-tier talent but also pushed me to keep learning every day. Your value as a leader is in recognizing and nurturing that talent, not competing with it.

## Technology

How you approach technology in your team is equally important to hiring the right folks in the team. It is also the area that requires the strongest business acumen. Our objective is to enable the organisation to achieve its mission, and selecting the right technology is a key factor in guaranteeing that outcome. I use the term "technology" very loosely here. Think of this as infrastructure providers, service providers (queues, databases, integrations etc.), tech stack etc., I'll try to break things down along with some concrete case studies.

### Language Stack

The larger your language stack, the larger the risk surface area. The less interoperable your engineers are, the more difficult things are to maintain. You should understand your business and select the language that closely matches the business requirements.

* Are you building a soft realime system with fault tolerance requirements? You're likely going to want to use Erlang/Elixir (or rely on the BEAM).
* Working on next-generation AI/ML? Likely want Python/Mojo.
* Working on systems security? Likely want Rust.

That said, this will never be a perfect overlap. There are situations where your stack of choice will not enable you to achieve your goals. For example, say you are once again an AI/ML startup and you're performing training tasks and curently use Python. You may want to be able to distribute these tasks in such a way that maximise CPU/GPU scheduling and lo and behold, the GIL completely gets in the way. In such a scenario, you should place some effort into circumventing the problem, even with workarounds to begin with. If the problem does not go away, and throwing (an acceptable amount) of money at it doesn't mitigate it, then you can start thinking about paradigm shift such as introducing another language better suited for this work (e.g., Rust).

### KIBS (Keep it Boring Stupid)

When building out your infrastructure or products, we have a tendency to reach for the newest item on the shelf. This isn't always wrong, but it something to careful of, _especially_ for core systems or products. You should ideally select technology that has been tried-and-tested in the past. Ignore that fancy vector-database and use postgres with `pgvector`. Ignore that fancy nosql database and use postgres with `jsonb`. Ignore that fancy time-series database and use postgres with arrays. You're likely catching my drift here. Now, that's not to say we should not explore what's out there. When it comes to R&D or greenfield projects, a good amount of time should be spent seeing what the latest available technology is and understanding if it can give us some competitive edge, while being fully aware of the risks when adopting it.

<details>
  <summary>Adopting Technology Case Study</summary>

> There are scenarios where you can get best of both worlds. I think of these are pieces of tech that builts upon, or integrates heavily with, establised technology. We had requirements for our greenfield projects that required both a time-series datastore and a highly-performant queue.
>
> #### TimecaleDB
>
> We had gone through a number of different time-series vendors and ultimately chose TimescaleDB. Why? Simply because of sat on top of the Postgres engine. We could dip in-and-out of hypertable land, to regular Postgres when we needed to. It meant we had the existing Postgres ecosystem while also being battle-tested. That's not to say Timescale was perfect, we had our troubles[^4], but all in all we were an up-and-coming startup which meant we got a lot of attention (with the aim of eventually closing a large deal). All in all, it was a great decision that has enabled us to process 10s of thousands of events per second with minimal compute (1 CPU/2GB mem)
>
>
> #### Redpanda
>
> Redpanda draws on the same thread. We needed a fast queue, and were likely going for Kafka. The operational overhead of managing Kafka is extremely high, and using a managed service would've cost a fortune. Redpanda was entirely compliant with the Kafka-API (missing features we did not need) and had minimal operational overhead as being deployed via a single binary. It was also exceptionally fast and tailored to modern day storage access patterns. Again, we were early and as they were running their Serverless product, which meant we were running free as part of the Beta programme for ≈8 months. That was a **huge** relief for our team and really gave us the space focus on building our product without having to worry about the queue.
>
> The underlying lesson I've learnt from this is to take calculated risks that give you an advantage but also minimise downside. You should also think about your company's public perception and see if you can leverage that to your advantage. If you are growing startup, you'll likely be able to negotiate good deals that benefit all parties. Both of these products are still in use today and likely will be for the coming years.

</details>

## Leaders Eat Last

As an EM, you occupy a unique position in the organisation. You're not quite executive level, nor are you purely operational. Instead, you're the connective tissue, the link between strategy and execution. At its core, your role is that of an enabler and a support function for your team. Your primary focus should be on cultivating an environment where your engineers can thrive. For each team member, constantly ask yourself:

1. How can they grow as engineers?
2. How can their personal growth align with our organizational goals?
3. What motivates them, and how can I help them stay engaged?
4. How can I inspire them to rally behind our vision?

Remember, your success in the eyes of the company is measured by your team's outcomes. Here are some key principles I try to follow and share with you.

* **Absorb failures, distribute successes**: When things go wrong, take responsibility. When things go right, ensure your team gets the credit. Cliché I know.
* **Be a political buffer**: Absorb organisational politics, but don't isolate them. Help them understand how their work impacts the company and users.
* **Engineer Independence**: Avoid creating dependencies on yourself. Your team **must** function smoothly even in your absence.
* **Spotlight your team**: Encourage your engineers to present their work to the company or customers. Writing posts, engaging in public marketing initiatives, all will help them be more involved and support them in future work.
* **Cultivate technical respect**: While your role is primarily about people, maintaining your technical skills is crucial. It helps you make informed decisions and earns the respect of your team.
* **On taking risk**: If you want to be in any position of leadership, you will need to take hard decisions. Those decisions will carry risk, so learning how to assess risk, _especially_ with partial information, is critical.

<details>
  <summary>Leadership Case Studies</summary>

> ### Political Buffer
>
>  I have recently become a father to my amazing son. Right after delivery, I had taken 2-weeks of parental leave. Before leaving, I made sure to have a good handover with my engineers and to write things down for them. What are my processes, how do certain parts of the platform work, and so forth. With my team prepared, I asked them to hold the fort (and so they did!). I still had the itch to ask how things were going but I hardly got any messages during my leave.
>
>  When I came back, one of my engineers pulls me into a quick-call to vent frustration and show appreciation for how much external energy was kept out of the team. To paraphrase, _"I seriously had no idea just how chaotic it is. Usually we have our syncs and we just focus on our work. You have no idea how much I appreciate you tackling that for us."_. I really appreciated that message, I never quite thought about it and just did it out of reaction.
>
> ### Spotlighting
>
> Whenever I see there's a period of time where an engineer does not have a lot of immediate work to do, I jump on the opportunity to move them to some work that is either more long-term but important, or I ask them to write about some of the work that they do in the form of an engineering blog post. I want them to have an external piece of work with their name attributed to it that highlights their competency and can benefit from peer-review to make sure it really shines. This is beneficial not only for the company and within the company, but even for the engineer should they leave in the future as it is a stamp of approval of the professional work that they do.
>
> ### Embracing Risk
>
> Given that I started when the company had no product, I had to make various architectural and design decisions about how our systems would be built. I was still a very early employee and there was a lot of doubt in my ability. The architectural path I chose was a bit unconventional at the time and involved a less-common pattern (CQRS[^5]). My goal was to keep this as dead-simple as possible and meet the companies goal. No EKS/K8s, no microservices, no esoteric tools. Single monorepo with Rust/Cargo workspaces and a modular monolith. This is usually scary as presenting things as simple might come off as lacking competence from folks who are.. less competent. In the end, I doubled-down on the approach; we hit, and exceeded, all the marks so much so that we adopted the same pattern for the second product (as it overlapped very well). All the engineers, including those not on my team, understood the approach and became familiar with how things worked. In the end, it all paid off (technically).

</details>


Lastly, don't forget to just have fun with your work. Software engineering is inherently creative (I believe one of the most creative art), and a playful environment often leads to innovative solutions. Keep things light when possible – not everything needs to be deadly serious. Your role as an EM is challenging but rewarding. You're not just managing projects or code; you're nurturing careers, and building a team that's greater than the sum of its parts. Embrace this responsibility, and you'll find it's one of the most fulfilling roles in tech.

# Conclusion

These reflections offer one view into the realities and challenges of engineering management, and I hope they serve as a guide to those on a similar path. Leadership is a journey, and like engineering itself, it’s an ever-evolving process of learning and adaptation. I hope you found at least a part of it interesting (especially if you made it to the end!). Would love to hear your thoughts, case studies or experiences. I can always include them in this post as well.

[^1]:[https://github.com/redis/redis/issues/5335](https://github.com/redis/redis/issues/5335)
[^2]:[what specifically are the zercost?](https://www.reddit.com/r/rust/comments/bo13qq/what_specifically_are_all_the_zerocost/)
[^3]:[Principles: Life & Work by Ray Dalio](https://www.goodreads.com/book/show/34536488-principles)
[^4]:[sqlx testing with timescaledb](https://blog.exein.io/sqlx_testing-blog-post-by-bogdan/)
[^5]:[CQRS by Martin Fowler](https://martinfowler.com/bliki/CQRS.html?ref=blog.funda.nl)
