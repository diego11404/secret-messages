# secret-messages

## 1. debemos construir la imagen con el siguiente comando y usando el archivo Dockerfile ubicado en la raíz del projecto.
<br>

> docker build -t secret-messages .

<br>

## 2. Ejecutar imagen usando la data de entrada ubicada en src/data.txt
<br>

> cat src/data.txt | docker run -i secret-messages 