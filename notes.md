# Internet Relay Printer

## software requirements

- needs to interface with the printer using one of the following
	- network
	- db-9 serial
	- USB
- needs to be able to run well on a Raspberry PI
- needs to take any type of data stream as input
- needs to be able to parse that data stream to Esc/Pos print commands
- needs to be able to cut paper automaticly to prevent printer errors
- needs to be able to shut off at moments notice
- needs to make the PI able to be accessed and managed without a screen
	- prob done by SSH and a VPN/VXLAN
- needs to be able to be modified and updated with minimal pain
	- prob done my a phisical buttion and git to be able to compile on demand
- needs to be ok enough code that I don't want to rip my hair out
- needs error handling that outputs to the printer
- needs to be able to connect to IRC
	- prob done with the IRC rust crate
	- will also need to check if connected to network
	- will also need to be able to get the PI to connect to network
		- I should prob use ethernet
			- wifi would be hell to setup
- needs to detect if the printer is connected and powered on



## power Requrements
- outputs
	- 5V  3A
	- 24V 2.5A
- theoretical inputs
	- USB-PD
		- would need a 120W cable for EPR at theoretcaly 28v
		- would also need some conversion from 28v to 5v and 24v
		- would have 28v at 2.7ish Amps
	- AC wall power
		- 120V
		- I would need to create a new cable
		- I would need to actualy deal with high voltage
		- could get me killed
	

## ideas for power

- use USB-PD
	- would need specal PCB
	- input would be 20V or 28v and not 24V
- use PoE
	- would need a network switch that has PoE++
	- would require that I bring Network switch to Open Sauce
	- would need specal PCB
	- would not work because not enough wattage
- screw with the provied cable to be able to also provide 5V
	- would need to figure out what is happening with the NC pin that is very clearly connected to 24V on the PCB
	- would be kinda dangerous
		- I would need to get this past TSA
		- could be a fire/Short risk
		- making the cable is going to be an absolute nightmare
	- would need to work with 120V AC
		- very dangerous
	- no garentee that it will work
- Bite the bullet and just use 2 cables
	- while easy this would feel like a cop-out and would be kinda sad
	- I would need to actualy route both of the cables

## Case Ideas

- effectively a long-ish tube that will have the PCBs spread throughout with the paper in the back reaching to the front
	- I am limited by the length of the ribben cable length
	- will need to be able to fit in a suitcase
	- there would be risk of paper jams if there is no channel for the paper
	- this would be a nightmare to reload
- pizza box design
	- will need a underhang area to hold the paper
	- could mount to desk relitively easy
	- would require quite a lot of 3D printer filiment
	- might not fit in suitcase
	- would be hard to access the IO
	- would be a pain to mount buttons
- THE CUBE
	- would look very ugly
	- could not mount to desk
	- very simular to the origional case
	- would not require too much 3D printing
		-  would only need to design the bottom part
	- not very fun and wimsy



## other notes

- No matter what I will need to redesign the case to fit both the recept printer PCB and the Raspberry PI and to be able to route the cables
- I will also need to write specal software to be able to connect to an IRC server and to be able to parse the data from IRC
- I could make a new thermal paper receptical to make the system a smidge more portable
- I would want/need the printer and PI to use 1 power switch
- I WANT FUN BUTTIONS
	- power
	- reset
	- feed
	- sync
	- status
	- test
	- fan
	- print meme (mayhaps)
	- connect SSH
		- could make this a cool comand board
		- it is my holodeck
- I would also need some indicator LEDs
	- power
	- error
	- network
	- panic
- I could also bedazle the case with lights and other fun things
- I should probably setup an IRC server at my home server
	- This would only be if there does not exist a good alternitive server
- might want to send in the IRC channel that I am monitering that the printer is active and reading.
- the paper holder will be at least 80mm wide and at least 80mm tall/diamiter
- if not I would need to manualy unroll the recept paper and try and wind it around my contraption

## I have decided on using a cube like design

# The Design

![](https://cdn.hackclub.com/019e3c0c-1e35-76a0-89a9-28ce9fe111c2/design.png)



# BOM


- 3223DB9RP1G1E : DB-9 Connector
- any 10 gauge wire (I would like 4 colours if I am able)
- 10 gauge wire crimp ends
- heat set inserts
- screws (I need to be able to dissaseble for airport security)
- 3223DB9RS1G1E : DB-9 female
- N238-001-BL : RJ-45 keystone





# DB-9 Pinout

### PI Power
1. 5v
2. gnd
### printer power
3. "N.C"
4. 24v
5. gnd



### bonus USB connection (I know this is crap but I have the pins)
6. usb 5v
7. usb d+
8. usb d-
9. usb gnd




