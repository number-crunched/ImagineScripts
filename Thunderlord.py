xval = [0]
yval = [62]
lightningtime = [0]
lightninga = [0]
totalbullet = [0]
totalbullettime = [0]

def initthunder():

        global lightninga
        global lightningtime

        global totalbullet
        global totalbullettime

        ammo = 62
        totshots = 0
        stage = 0
        time = 0
        tottime = 0
        lightning = 0
        rs0 = (1/(460/60))
        rs1 = (1/(620.68/60))
        rs2 = (1/(704.34/60))

        while ammo > 0:
            totshots += 1
            if stage == 0:
                time += rs0
                tottime += rs0
                ammo -= 1
            elif stage == 1:
                time += rs1
                tottime += rs1
                ammo -= 1
            else:
                time += rs2
                tottime += rs2
                ammo -= 1
            
            if time >= 1:
                time = 0
                ammo += 7
                lightning += 1
                lightninga.append(lightning)
                lightningtime.append(tottime)
            
            if tottime >= 0.96 and tottime < (2.26+0.96):
                stage = 1
            else:
                stage = 2

            global xval
            global yval

            totalbullet.append(totshots)
            totalbullettime.append(tottime)

            xval.append(tottime)
            yval.append(ammo)
            
initthunder()

print("Total Rounds Fired: " + str(totalbullet[len(totalbullet)-1]))
print("Total Lightning Strikes: " + str(lightninga[len(lightninga)-1]))
print("Total Time: " + str(totalbullettime[len(totalbullettime)-1]))