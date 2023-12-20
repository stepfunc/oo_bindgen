class BindingLibraryLoader {
    static class Target {
        final String directory;
        final String name;
        final String extension;

        Target(String directory, String name, String extension) {
            this.directory = directory;
            this.name = name;
            this.extension = extension;
        }

        String path() {
            return "/" + this.directory + "/" + this.name + "." + this.extension;
        }
    }

    static void loadTargets(Target[] targets) {
        if (targets.length == 0) {
            throw new RuntimeException("No targets were specified");
        }
        Throwable lastException = null;
        for (Target target : targets) {
            try {
                loadTargetLibrary(target);
                return;
            } catch (Throwable ex) {
                lastException = ex;
            }
        }
        throw new RuntimeException("Unable to load a target shared library", lastException);
    }

    private static void loadTargetLibrary(Target target) throws Exception {
        final String path = target.path();
        java.io.InputStream stream = NativeFunctions.class.getResourceAsStream(path);
        if (stream == null) {
            throw new Exception("Resource not found: " + path);
        }
        java.nio.file.Path tempFilePath = java.nio.file.Files.createTempFile(target.name, "." + target.extension);
        tempFilePath.toFile().deleteOnExit();
        java.nio.file.Files.copy(stream, tempFilePath, java.nio.file.StandardCopyOption.REPLACE_EXISTING);
        System.load(tempFilePath.toAbsolutePath().toString());
    }
}
