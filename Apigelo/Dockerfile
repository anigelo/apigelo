FROM mcr.microsoft.com/dotnet/aspnet:6.0 AS base
WORKDIR /app
EXPOSE 80
EXPOSE 443

FROM mcr.microsoft.com/dotnet/sdk:6.0 AS build
WORKDIR /src
COPY ["Apigelo/Apigelo.csproj", "Apigelo/"]
RUN dotnet restore "Apigelo/Apigelo.csproj"
COPY . .
WORKDIR "/src/Apigelo"
RUN dotnet build "Apigelo.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "Apigelo.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "Apigelo.dll"]
