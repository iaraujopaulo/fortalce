# FortalCE - como gerar os instaladores (sem instalar nada no seu computador)

Voce nao precisa compilar nada. O GitHub faz isso por voce, de graca.

1. Crie uma conta em github.com e clique em "New repository" (nome: fortalce, publico ou privado).
2. Em "uploading an existing file", arraste TODO o conteudo desta pasta (inclusive a pasta oculta .github).
   Se a .github nao subir: Add file > Create new file, digite `.github/workflows/build.yml` e cole o conteudo do arquivo.
3. Abra a aba "Actions", escolha "Construir instaladores FortalCE" e clique em "Run workflow".
4. Aguarde 20 a 40 minutos. Se algum job ficar vermelho, copie o erro e me envie.
5. Windows, Linux e Mac: baixe os instaladores em "Releases" (rascunho). Android: abra a execucao e baixe o APK em "Artifacts".

## iPhone
So e possivel com um Mac, o Xcode e uma conta Apple Developer paga (US$ 99/ano). Sem isso a Apple nao deixa instalar apps.

## Modo Internet Explorer (Windows)
Edite SITES_LEGADOS em src-tauri/src/lib.rs com os enderecos antigos. Esses sites abrem no Edge em modo IE,
que precisa estar habilitado no Windows (politica "Configurar integracao com o Internet Explorer").
