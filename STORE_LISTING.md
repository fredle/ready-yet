# Ready Yet - Microsoft Store listing

## Product name

Ready Yet

## Short description

See what your PC is starting at sign-in, what is still to come, and how long until it is usable.

## Description

After you sign in, your PC spends a minute or more starting apps and services in the background, and Windows shows you nothing about it. You wait, you guess, and you never find out which program is costing you the time. Task Manager's Startup apps page lists what is set to start, not what has started, what is still to come or when the PC will settle.

Ready Yet watches the startup itself. At sign-in a small window shows how many startup items have started ("9 of 15 started"), CPU, memory and disk activity, and the time left until the PC is usable. When everything has started and the PC has been quiet for a few seconds, it saves the timings and hides. If an app has still not started, Ready Yet keeps watching for it from the notification area for up to ten minutes, then closes, so it costs nothing for the rest of the day.

After each boot you get a report:
- How long the PC took to become usable, against your usual time over the last ten boots.
- A timeline of what started when.
- The apps that used the most processor time and took longest to settle, such as "Teams cost you 11 s".

Then you can act on it. For apps set up for your account, you can turn them off at sign-in, or hold them back so they start only once the PC is ready. Turning an app off uses the same switch as Task Manager, so it is easy to undo. You can also end an app that is running.

Ready Yet looks at the Run and RunOnce registry keys, both Startup folders, scheduled tasks that run at sign-in, and automatic services. Services are listed read-only, each with a one-line note on what it is.

By default Ready Yet starts with your other startup apps and needs no administrator rights. It rebuilds what happened before it opened from each program's start time and processor use, and tells you that programs which started and closed before then are missing. If you want the full picture, turn on Start earlier: it asks for administrator approval once and then watches from the first second of sign-in, and lets you stop an app as soon as it starts.

The time-left estimate comes from your own earlier boots. On the first boot Ready Yet shows no estimate rather than a guess.

Nothing leaves your PC. There is no account and no telemetry.

## Product features

- Small window at sign-in with a started count, CPU, memory and disk activity, and the time left
- Live status for each startup item: waiting, starting or started
- Report after every boot: time until usable, compared with your last ten boots
- Timeline of what started when, up to the moment the PC was usable
- Shows which apps used the most processor time and disk during startup
- Time-left estimate learned from your own boots, with no guess on the first one
- Turn an app off at sign-in using the same switch as Task Manager
- Hold an app back until the PC is usable, then Ready Yet starts it
- End a running startup app from the list
- Covers Run keys, Startup folders, sign-in scheduled tasks and automatic services
- Services listed read-only with a short note on what each one is
- Works without administrator rights; optional Start earlier mode watches from the first second
- Keeps watching for apps that start late, for up to ten minutes after sign-in
- Notification area icon with progress in its tooltip; click it to open the report
- Closes itself once startup is over
- Lists itself among the startup items with its own measured cost
- Light and dark themes that follow Windows

## Search terms

- startup apps
- boot time
- slow startup
- startup monitor
- sign-in speed
- startup programs
- delay startup

## Category and subcategory

Category: Utilities & tools
Subcategory: System utilities (or "Other" if not offered)

## Privacy

Data stored locally, in %LOCALAPPDATA%\ReadyYet:
- settings.json: your options, and the apps you held back or turned off.
- boots.json: startup timings of your last 30 boots (item names, start times, processor time and disk use).

Ready Yet also adds a "Ready Yet" entry to your account's Run registry key so it opens at sign-in, or, with Start earlier on, a sign-in scheduled task. Both are removed when you turn the option off.

Network: the only network use is the update check. About once a day, if "Automatically update" is on, Ready Yet asks GitHub Releases whether a newer version exists and downloads it. No usage data, process lists or personal information is sent anywhere. There is no telemetry and no account.

## Screenshots

1. See your PC start up - what has launched, what is still to come, and roughly how long is left.
2. What started when - a timeline of every startup app, up to the moment the PC was usable.
3. Find what slows you down - each app's processor time and disk use during startup, worst first.
4. Start apps once ready - hold an app back until the PC is usable, or turn it off at sign-in.
