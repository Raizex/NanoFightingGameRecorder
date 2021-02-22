Recording Video Games With Jetson Nano
======================================
1. Executive Summary
--------------------
The goal of this project is to develop a system for quickly, cheaply, and easily capturing footage from multiple matches at a local fighting game tournament.

2. The Problem
--------------
Local fighting game tournaments usually have anywhere from 4-40 video game consoles setup for players to play matches on. These Playstation 4's are brought in by players and volunteers, and let 2 players at a time play their matches in a double elimination tournament. Some of these tournaments have one of these stations set up as a recording and brodcasting station for streaming live to Twitch.tv. Unfortunately, these recording and broadcasting stations can be quite costly to put together, requiring several thousand dollars of computer, audio, and video equipment that takes a significant amount of time and space to setup properly for recording, and nearly constant monitoring and management by one or more volunteers.

Playing on this streaming station can be extremely vauluable for players, since they can later go back and review their matches, letting them much more easily identify where and how they can improve in the future.

Unfortunately, due to the nature of there only being a single streaming station, not many players get to play on stream, with high-profile matches between strong players usually taking priority. This leaves less-skilled players without a valuable tool for improving their skills.

Ideally, every match on every station would, at the very least, be recorded and later uploaded to YouTube. Unfortunately, it isn't practical to set up each of these stations with a high-end streaming computer equiped with a video capture card, as is currently done for the streaming station. Thus, a different solution is needed.

3. Purpose and Goals
--------------------
The purpose of this project is to develop a hardware and software system capable of recording video footage taken from multiple Playstation 4s at the same time and then later upload that footage to YouTube.

The system should have the following characteristics:

- Introduce little to no delay in the HDMI signal path from the console to the monitor. (Anything more than an additional 5ms delay is unlikely to be acceptable to tournament players.)
- Capture video in 1080p at 60 frames per second.
- Be small and portable.
- Be quick and easy to setup, operate, and tear down.
- Be relatively inexpensive.
- Be able to record anywhere from 4 to 40 Playstations at the same time.

4. Team Members and Roles
-------------------------
- Justin Huffman (Customer/Developer)
- Tanner Rosner (Developer)

5. Minimum Viable Product
-------------------------
A minimum viable product will be able to split the HDMI output from a PS4 and send one copy to the screen with minimal delay (less than 5 milliseconds). It will then send the other copy into a Jetson nano via an HDMI to CSI-2 bridge adapater to the Jetson Nano's CSI-2 camera input. Players will then be able to start and stop recordings on the Jetson nano using an Android tablet connected via Ethernet. An administrator will then be able to collect footage from several such devices using a networked computer, to be then uploaded to YouTube at a later date.

6. Stretch Goals
----------------
Automating the proccess of video aggregation and upload would be extremely useful, if not entirely neccesary.

Smash.gg integration could be useful for keeping track of which players are in which videos, which would help players much more easily find their match videos.

A custom PCB and enclosure with integrated HDMI to CSI-2 bridge adapters would help cut long-term costs, as well as make the system easier to setup and manage.

7. Deliverables
---------------
The following deliverables will need to be produced:

- Extensive documentation for setup and installation of the hardware part of the system, including parts and assembly instructions.
- A rust applictaion that runs on the Jetson Nano to control and label recordings, as well as expose an API for interfacing with the UI.
- An Android application that runs on an Android tablet that exposes a touch screen interface to the players for startnig and stopping recordings.
- A command and control application that runs on a networked computer for aggregating and uploading footage collected by multiple Jetson Nanos.

8. Deployment Plans
-------------------
Since the system is designed to be portable, deployment will not be automated outside of testing, as the system will need to be portable and setup on-site whenever it is used.

Thus, thourough documentation on setup and installation procedures will need to be produced. Ansible may be a good choice for automating as much as possible of the setup and isntallation process as possible, but further research is needed as to which tool(s) would be best suited for this task.

9. Risks
---------
There are several unkowns that need to be resolved for this project to work, including:

- Does the chosen hardware work together in the manner described?
- Is the hardware capable of recording video in real-time with acceptable quality?
- Is it possible to meet the minimal lag constraint?
- How will we work together on a hardware project while remote?
