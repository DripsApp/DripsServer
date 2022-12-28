FROM frolvlad/alpine-glibc
COPY pack /home/
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories && apk add --no-cache -U libgcc
RUN chmod +x /home/app
WORKDIR /home
EXPOSE 9000
ENTRYPOINT ["/home/app"]