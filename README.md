# secret-messages

## Primero debemos construir la imagen con el siguiente comando y usando el archivo Dockerfile ubicado en la raíz del projecto.
#

> docker build -t secret-messages .

## Ejecutar imagen usando la data de entrada ubicada en src/data.txt
#
> cat src/data.txt | docker run -i secret-messages 