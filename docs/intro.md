j'essay de parser des trames profinet en rust. 

Y'a 0 documetnation claires sur internet pour les differente structure de pacquet reseau profinet. 
Je sais qu'il ya plusieur type de profinet dont 2 bien distainct : Profinet RT qui se trouve sur la couche 3 et Profinet Asynchrone qui se trouve sue la couche 7. 

Ce Mémoire de projet de fin d’études semble donné une explication de la difference entre les deux protocol. l'un permet la comunication rapide et l'autre permet le parametrage : 
https://repository.enp.edu.dz/jspui/bitstream/123456789/9941/1/YAHIA%20AISSA.Ilyas.pdf


Apres avoir retier un server et metre branché à ca place : je vois des trame dcp :
la doc dit ça :
2.5 Adressage des appareils
PROFINET
Les appareils Ethernet communiquent toujours en
utilisant leur adresse MAC unique (voir encadré).
Dans un système PROFINET, chaque appareil de
terrain reçoit un nom symbolique qui identifie de
manière unique l'appareil de terrain au sein du
système d'E/S (Figure 9). L'appareil est identifié et
configuré avec ce nom dans le cadre du processus
d'ingénierie. Les adresses IP et MAC exactes sont
résolues à l'aide de ce nom lorsque l'application
PROFINET est lancée.
Le protocole DCP (Discovery and basic Configuration
Protocol) est utilisé à cet effet. Le nom de l'appareil
est attribué à l'appareil d'E/S individuel et donc à
son adresse MAC par un outil d'ingénierie utilisant
le protocole DCP lors de la mise en service
(initialisation de l'appareil). En option, le nom peut
également être automatiquement attribué par le
contrôleur d'E/S au dispositif d'E/S au moyen d'une
topologie spécifique basée sur la détection de
voisinage. L'attribution de l'adresse IP se fait à partir
du projet via le DCP, en utilisant le DHCP (Dynamic
Host Configuration Protocol), généralement
commun et répandu au niveau international, ou en
utilisant des mécanismes propres au fabricant.
Dans ces deux derniers cas, les adresses IP sont lues
automatiquement par le contrôleur via DCP, de
sorte qu'aucun accès manuel n'est nécessaire ici
non plus. Les options prises en charge par un
appareil de terrain sont définies dans le fichier GSD
de l'appareil de terrain concerné.

Apres avoir retier un cable rj-45 cpu1 vers CPU0 qui etait en double j'ai vue des trames MRP sur wireshark : 
la doc : profinet_descriptionsysteme_dec_2018_fr.pdf page 18 decrit ceci :
Protocole de redondance des supports (MRP)
Le protocole MRP selon la norme CEI 62439-2 décrit
la redondance de PROFINET avec un temps de
reconfiguration typique de moins de 200 ms pour
les chemins de communication avec des trames
TCP/IP et RT après un défaut. Le fonctionnement
sans erreur d'un système d'automatisation
implique un Responsable de redondance des
supports (MRM) et plusieurs Clients de redondance
des supports (MRC) disposés en anneau
