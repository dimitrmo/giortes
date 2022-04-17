# Giortes

## Lessons learned

* Simple RSS parse using rss crate
* Use of Arc when you require shared data across threads
* No need for whole scheduler for a simple cron task. Just a spawned thread
* For mutability of shared data locks are required
* Replace std rwlocks with futures-locks to avoid errors when sharing data across threads safely
