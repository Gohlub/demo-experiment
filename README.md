## Work in Progress

 Tests are not implemented yet (working on figuring them out and generating docs, also need to move the tests into the https://github.com/hyperware-ai/hyperprocess-macro).

## Context management
In `/resources`, amongst others, there are two folders, `/app-framework-guide.md` and `/llm-generated-app-guide`. The former is just a copy-paste of from the hyper-process-macro README on how to use the app framework, and the latter was generated by Claude Code. Make sure to get rid of one or the other when prompting, since they overlap and might bloat the context window. 

On the topic of others, I just included things that might be might be useful, such as contact management, kit, and the use of hyper-bindgen. In the initial prompt, I usually emphasize this folder as a reference and explicitly reference hyper-bindgen to make sure the LLM client makes use of it.


The `load-balancer-example` branch includes a WIP for a load-balancer implementation (through Claude Code). I encountered an issue with imports and hyper-bindgen that I escalated with Luc, so it doesn't compile yet.


