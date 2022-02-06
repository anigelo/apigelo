using Apigelo.Services;
using Microsoft.AspNetCore.Mvc;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddSingleton<FileService>();

var app = builder.Build();

app.MapGet("/anime", ([FromServices] FileService service) => service.GetAnimes());

app.Run();