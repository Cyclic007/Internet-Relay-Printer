## Internet Relay Printer

# may 17-18 (I was doing this on a red-eye flight)

I wrote up the requiremts and made diagrams of each of my theoretcal designs

![](https://cdn.hackclub.com/019e3ba1-a6c6-7ebf-bb43-f92f68988f5d/thetube.png)
![](https://cdn.hackclub.com/019e3ba2-76bb-75d3-bb34-2c1bc3c64f8a/theCube.png)
![](https://cdn.hackclub.com/019e3ba2-c95f-7f4f-b3f3-d40c8796d702/otherCube.png)


## time spent (about 3 hours)

# may 18th (for real this time with slightly more sleep)

I finaly decised on THE CUBE as the final design and made a diagram to show the positons of everything
![](https://cdn.hackclub.com/019e3c0c-1e35-76a0-89a9-28ce9fe111c2/design.png)
thoughout all of this I have been updating notes.md to keep track of the ideas and thought prossess

I am going to be effectively required to use USB-PD EPR to be able to draw 28v at 5 amps while this is a lot of wattage (140 watts to be exact) I will not be using all of it (hopefuly).


# May 20/21 (Trying to figure out power)

I am going to make a 5 pin "adapter" that will have 2 ground pins 1 5v pin 1 24v pin and the weird as crap N.C pin on the Recept printer that is very clearly connected and has 24v

I am going to need to use at least 10 gauge wire so that It wont try and burn everything to the ground

I will also be ueing a db-9 connector very incorrectly with the top 5 pins almost exclusively and I will figure out what to do with the bottom pins.

I might have a USB signal go though the bottem 4 pins we will see

## time spent (about 5 to 6 hours)

# May 30

I have started to design the case 

I am prototypeing the system that I will use and I have gotten the pcb out of the case it came in

![](https://cdn.hackclub.com/019e7a15-2a85-726b-8723-3f58898a2249/img_8067.jpg)
![](https://cdn.hackclub.com/019e7a15-2cb3-7e68-9f80-7e021a660fed/img_8066.jpg)
![](https://cdn.hackclub.com/019e7a15-2ef2-7873-b062-52240608a0bc/img_8065.jpg)
![](https://cdn.hackclub.com/019e7a15-3102-7afa-9b80-d15cf2835487/img_8063.jpg)
![](https://cdn.hackclub.com/019e7a15-3322-7a03-a38c-7095fde63444/img_8062.jpg)
![](https://cdn.hackclub.com/019e7a15-3539-7bea-8fbe-959e640b1d3f/img_8061.jpg)
![](https://cdn.hackclub.com/019e7a15-3779-7945-a32d-d8a1e9aa2dce/img_8060.jpg)
![](https://cdn.hackclub.com/019e7a15-3d9c-7c56-abbc-4a64d2820e40/img_8069.jpg)
![](https://cdn.hackclub.com/019e7a15-3b90-75c2-83b7-d9b4886bb9c4/img_8070.jpg)
![](https://cdn.hackclub.com/019e7a15-3992-7a31-8688-b636c67339cb/img_8059.jpg)


I made the full case amd mow have a 35 hour 3d printing job going

# time spent (about 8 hours)


# May 31 - June 2nd (Finishing up)



I fixed the case so  that I can actualy get all of the pcbs in
I setup the raspberry pi to have all terminal output go directly to the recept prinnter so that I can just hook it up to a live cli irc client.
The commands that are run at startup are 

- script -f /dev/usb/lp0
- export PROMPT_COMMAND='echo "\n\n\n\n\n\n\n"'

These commands will pipe all terminal output to the printer LIVE

![](https://cdn.hackclub.com/019e88d2-5e7f-7f5c-873b-45366a2b8c01/img_8097.jpg)


## Time spent (about 4 hours)
