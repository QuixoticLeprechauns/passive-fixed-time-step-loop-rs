# passive-fixed-time-step-loop-rs
A minimalistic fixed time step program loop that doesn't need to actively calculate required sleep times.

# How Does it Work?
This unique loop construction uses multithreading to achieve a fixed time step, at little cost to system performance. This loop design intentionally avoids CPU intensive lag processing, and spinlocking. It asks for a seperate thread to sleep for the duration of your target time step once each cycle of the loop, while runniung a process on the main thread, followed by a seperate, manditory, minimum sleep period to account for possible lag. The loop haults it's progression during each cycle until the process finishes running, and sleeping thread joins the main thread.

# Disadvantages
 - This loop does not calculate time deltas. This results in lower precission compared to loops that use spinlocking.
 - Sleep granularity is system dependent. Because this loop uses sleeping to achieve a fixed time step, there is no way to improve it's timing accuracy.

# Credit to [people]
Although based on [citation], I am not the author of this variation of the passive-fixed-time-step-loop. Credit belongs to [people] for it's authorship.

